extern crate amethyst;

#[cfg(test)]
mod tests {
    use amethyst::{
        prelude::*,
        renderer::{ColorMask, DrawFlat, PosTex, ALPHA},
        Result,
    };

    #[test]
    fn segfault() {
        assert!(run().is_ok());
    }

    fn run() -> Result<()> {
        let assets_dir = format!("{}/examples/assets/", env!("CARGO_MANIFEST_DIR"));
        Application::new(assets_dir, TestState, game_data()?)?.run();

        Ok(())
    }

    fn game_data<'a, 'b>() -> Result<GameDataBuilder<'a, 'b>> {
        let path = format!(
            "{}/resources/display_config.ron",
            env!("CARGO_MANIFEST_DIR")
        );
        GameDataBuilder::default().with_basic_renderer(
            path,
            DrawFlat::<PosTex>::new().with_transparency(ColorMask::all(), ALPHA, None),
            false,
        )
    }

    #[derive(Debug)]
    struct TestState;
    impl<'a, 'b> State<GameData<'a, 'b>> for TestState {
        fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
            data.data.update(&data.world);
            Trans::Quit
        }
    }
}
