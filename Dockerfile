# Start from rust offical docker image 
FROM rust:1.70
# Set install shell as bash
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
# Add Rust to PatH
ENV PATH="/root/.cargo/bin:${PATH}"
# Add Neovim to PatH
ENV PATH="/root/.local/share/bob/nvim-bin/:${PATH}"
# Add Nu to PatH
ENV PATH="/root/.local/share/nu:${PATH}"

##################
# Install basics #
##################
# Setup Substrate Enviourment   : https://docs.substrate.io/install/linux/
# Reference Parity Dockerfiles  : https://github.com/paritytech/scripts/blob/master/dockerfiles/contracts-ci-linux/Dockerfile

RUN  apt-get update -y && \
  apt-get upgrade -y && \
  apt-get install -y --no-install-recommends cmake build-essential git clang curl libssl-dev protobuf-compiler && \
  apt-get install -y --no-install-recommends zlib1g-dev npm wabt jq &&\
  curl -L $(curl --silent https://api.github.com/repos/WebAssembly/binaryen/releases \
		 | jq -r '.[0].assets | [.[] | .browser_download_url] | map(select(match("x86_64-linux\\.tar\\.gz$"))) | .[0]' \
		 ) | tar -xz -C /usr/local/bin/ --wildcards --strip-components=2 'binaryen-*/bin/wasm-opt' &&\
 rustup install stable && \
 rustup target add wasm32-unknown-unknown --toolchain stable && \
 rustup component add rust-src --toolchain stable && \
 rustup default stable && \
 rustup toolchain install nightly-2023-03-21 --target wasm32-unknown-unknown --profile minimal --component rustfmt clippy rust-src &&\
 ln -s "/usr/local/rustup/toolchains/nightly-2023-03-21-x86_64-unknown-linux-gnu" /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu && \
 cargo install cargo-dylint dylint-link sccache &&\
 cargo install cargo-contract  && \
 cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git && \
 apt-get clean && rm -rf /var/lib/apt/lists/*

###########################
# Install Frank Dev Utils #
###########################
# Inspiration from YT Guy - Boilerplate                       : https://github.com/0atman/noboilerplate/blob/main/scripts/20-rust-userland.md#oxidise-your-life-1
# Add Neovim(AstroVim) via bob-nvim & setup starship for bash : https://astronvim.com/
# Add zellij                                                  : https://zellij.dev/documentation/
# Add nu shell                                                : https://www.nushell.sh/book/coming_from_bash.html
# Setup NuShell with starship                                 : https://www.nushell.sh/book/configuration.html https://starship.rs/guide/

RUN cargo install cargo-info ripgrep exa bat irust bacon \
    gitui rtx-cli starship bob-nvim kondo aichat && \
    bob install 0.9.1 && \
    bob use 0.9.1 && \
    git clone --depth 1 https://github.com/AstroNvim/AstroNvim ~/.config/nvim && \
    nvim --headless "+Lazy! sync" +qa && \
    echo 'eval "$(starship init bash)"' >> ~/.bashrc && \
    curl -LOJ https://github.com/zellij-org/zellij/releases/download/v0.37.2/zellij-x86_64-unknown-linux-musl.tar.gz && \
    tar -xvf ./zellij-x86_64-unknown-linux-musl.tar.gz && \
    mv ./zellij /usr/local/cargo/bin/ && \
    rm ./zellij-x86_64-unknown-linux-musl.tar.gz && \
    curl -LOJ https://github.com/nushell/nushell/releases/download/0.83.1/nu-0.83.1-x86_64-unknown-linux-gnu.tar.gz && \
    tar -xvf ./nu-0.83.1-x86_64-unknown-linux-gnu.tar.gz && \
    mv nu-0.83.1-x86_64-unknown-linux-gnu nu && \
    mv nu /root/.local/share/ && \
    rm ./nu-0.83.1-x86_64-unknown-linux-gnu.tar.gz && \
    mkdir ~/.config/nushell && \
    touch ~/.config/nushell/env.nu && \
    touch ~/.config/nushell/config.nu && \
    mkdir ~/.cache/starship && \
    touch ~/.cache/starship/init.nu && \
    starship init nu >>  ~/.cache/starship/init.nu && \
    touch ~/.config/starship.toml && \
    starship preset pastel-powerline > ~/.config/starship.toml && \
    echo "source-env ~/.cache/starship/init.nu" >> ~/.config/nushell/env.nu && \
    echo "\$env.config = { show_banner:false, edit_mode: nvim }" >> ~/.config/nushell/config.nu && \
    echo 'eval "$(rtx activate bash)"' >> ~/.bashrc &&\
    rtx install node@latest && rtx use node@latest &&\
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Start Zellij  
CMD ["zellij", "options", "--default-shell", "nu", "--session-name", "Rusty"]
