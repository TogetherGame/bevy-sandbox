use bevy::prelude::*;

const HELLO_WORLD: &str = "Hello World!!!";

fn main() {
    // 每一个 Bevy 项目可以看作为一个使用 ECS 架构的 App
    App::new()
        // plugins 为加载项，加载项可以理解为包含 Entity(实体),Component(组件),System(系统) 的集成模块
        // Bevy 提供了集成基础功能的模块作为 DefaultPlugins, 它包含了用于处理交互，图像，音频等等基础功能，
        // 为了方便，可以直接添加 DefaultPlugins 来支持这些功能。
        .add_plugins(DefaultPlugins)
        // resource 可以理解为一种特殊的 Entity(实体) 类型，可以把它想想为程序全局实体变量
        // 这里的 ClearColor 是 bevy 用作纯色背景使用的实体，而 ClearColor(Color::White) 意味此程序的背景为纯白色
        .insert_resource(ClearColor(Color::WHITE))
        // System(系统) 表示用于在特定情况下处理特定逻辑的函数，其中 `Startup` 表示程序开始运行时会执行的函数，
        // 而普通的 Update systems 将会在程序运行后的每个循环都运行一次。
        // 这里只使用了这两种常见 system, 但其实 bevy 支持通过 AppState 指定函数触发时机
        .add_systems(Startup, (welcome_system, show_hello))
        .add_systems(Update, animate_text)
        // 在定义完毕之后，需要执行 `run` 才能成功运行程序
        .run();
}

/// 只是在控制台内打印 "Welcome!!!"
fn welcome_system() {
    println!("Welcome!!!")
}

/// 这是一个 Component(组件) 的声明，它仅为一个 Unit Struct 类型，不包括任何数据，因此在这里它仅会作为 "标签" 的形式被赋予给其它实体
#[derive(Component)]
struct HelloText;

/// 添加 Camera 并且添加 "Hello World!" 文字
fn show_hello(mut cmd: Commands, assets: Res<AssetServer>) {
    // Commands 指程序运行前的"大管家"，可以借助 commands 来管理程序运行时将需要的所有实体, 包括 Resource
    // 这里使用了 spawn 添加了游戏必要组件 "Camera2d", 也就是用于显示游戏内一切的实体，缺少 Camera 的话
    // 游戏运行后只会显示黑屏。
    cmd.spawn(Camera2dBundle::default());

    // 添加 Text2d bundle, bundle 表现为组件的集合，Text2dBundle 就包含了 2D 文字组件(Text)，控制位移形变的组件(Transform)等等.
    // 这里只初始化了显示的文字，字体，大小，字体颜色，其余属性使用 `default()` 默认的定义。
    cmd.spawn(Text2dBundle {
        text: Text::from_section(
            HELLO_WORLD,
            TextStyle {
                font: assets.load("../../assets/fonts/GnuUnifontFull.ttf"),
                font_size: 40.0,
                color: Color::BLACK,
            },
        ),
        ..Default::default()
    })
    // 插入之前定义的 Unit Struct `HelloText` 组件，作为"标签"
    .insert(HelloText);
}

/// 这个 system 会逐帧改变所有带有 [`HelloText`] 组件的实体的位移，达到一种滚动文字的效果。
fn animate_text(
    // Query 代表用于检索的集合，比如这里的 `Query<&mut Transform, With<HelloText>>` 就表示：
    // 场景中所有带(With) HelloText 标签的实体的形变(Transform)信息
    mut hello_text: Query<&mut Transform, With<HelloText>>,
    // 获取窗口信息
    windows: Query<&Window>,
) {
    // 由于此程序仅定义一个窗口，所以只获得主窗口的相关信息就够了，
    let primary = windows.single();

    for mut transform in &mut hello_text {
        transform.translation.x += 1.;

        if transform.translation.x > primary.width() / 2. {
            transform.translation.x = -primary.width() / 2.;
        }
    }
}
