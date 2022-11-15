[English](./README.md)

# bevy-sandbox
一个使用 Bevy 引擎，并由 Rust 编写的游戏项目集合。

## 开始

1. 安装 Rust (使用 [rustup](https://rustup.rs/))

2. (Visual Studio Code 用户可选) 安装 rust-analyzer 插件及 rust-src:

    ```console
    rustup component add rust-src
    ```

    And

    !["analyzer"](./readme_res/rust-analyzer.PNG)

3. 克隆此代码仓:

    ```console
    git clone https://github.com/TogetherGame/bevy-sandbox.git
    cd bevy-sandbox
    ```

4. (Linux 用户) 安装依赖:

    - `gcc`.

    - `alsa-lib` 开发工具, 例如 `alsa-lib-devel` (RedHat 类).

    - `libudev` 开发工具, 例如 `systemd-devel` (RedHat 类), `libudev-dev` (Debian 类).

5. 由于此仓库为一个 Rust workspace, 为了执行 workspace 下的某一项目，你需要先 `cd` 至项目目录并执行 `cargo run` 或以此方式在仓库主目录直接运行某一项目 (以 hello_world 为例):

    ```console
    cargo run -p hello_world
    ```

    此项目提供了更为简便的 alias: `rp` , 与 `run -p` 功能相同，所以也可以直接执行以下命令运行，详见: [项目中 cargo_config 的 alias 项](./.cargo/config):

    ```console
    cargo rp hello_world
    ```

    > 首次运行可能会耗时数分钟，因为 cargo 需要联网获取 Bevy 引擎构建需要的所有构建依赖并执行构建.

## 参与贡献

1. Fork 此仓库(推荐) 或在此仓库下创建新分支

2. 克隆已经 Fork 的仓库或者此仓新创建的分支

3. 添加你的修改

4. 执行代码格式化及静态检查

    (需要 rustfmt 及 clippy, 若未安装则需要执行 `rustup component add rustfmt clippy` 安装)

    ```console
    cargo fmt --all
    cargo clippy --all
    ```

5. 提交 & 推送代码修改

    ```console
    git add .
    git ci -a -m '<提交信息>'
    git push
    ```

6. 最后根据已修改的分支提交代码合入申请 (Pull Request)
