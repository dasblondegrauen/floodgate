pub mod stage {
    pub trait Generator {
        fn get_image(&self);
    }

    pub trait Renderer {
        fn render_command(&self);
    }

    pub trait Sender {
        fn send_tcp(&self);
    }
}
