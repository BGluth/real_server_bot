use reals_server_bot_common::types::GameSetData;

#[derive(Debug, Default)]
pub struct GameSetMessageParser {}

impl GameSetMessageParser {
    pub fn extract_game_set_data_from_discord_msg_if_any(&self, msg: &str) -> Option<GameSetData> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use reals_server_bot_common::types::{GameSetData, PlayerInfoForSetBuilder, SetType};

    use crate::game_msg_parser::GameSetMessageParser;

    struct TestCase {
        input: &'static str,
        expected: Option<GameSetData>,
    }

    impl TestCase {
        fn new(
            input: &'static str,
            expected_p1_info: &PlayerInfoForSetBuilder,
            expected_p2_info: &PlayerInfoForSetBuilder,
            expected_set_type: SetType,
        ) -> Self {
            let expected = GameSetData {
                p1_info: expected_p1_info.clone().build().unwrap(),
                p2_info: expected_p2_info.clone().build().unwrap(),
                set_type: expected_set_type,
            };

            Self {
                input,
                expected: Some(expected),
            }
        }

        fn new_parse_fail_case(input: &'static str) -> Self {
            Self {
                input,
                expected: None,
            }
        }

        fn run_test(&self, parser: &GameSetMessageParser) {
            let parsed_res = parser.extract_game_set_data_from_discord_msg_if_any(self.input);

            if parsed_res != self.expected {
                panic!("Failure on {} (Expected: {:#?})", self.input, self.expected);
            }
        }
    }

    #[test]
    fn msg_parse_tests() {
        let tests = vec![
            TestCase::new(
                "Fluzzard Kazuya 5 - 0 Yoshi ./rust_man",
                PlayerInfoForSetBuilder::default()
                    .name("./rust_man")
                    .score(0)
                    .character("Yoshi"),
                PlayerInfoForSetBuilder::default()
                    .name("Fluzzard")
                    .score(5)
                    .character("Kazuya"),
                SetType::Ft5,
            ),
            TestCase::new(
                "./rust_man Yoshi 3 - 2 Kazuya Fluzzard",
                PlayerInfoForSetBuilder::default()
                    .name("./rust_man")
                    .score(3)
                    .character("Yoshi"),
                PlayerInfoForSetBuilder::default()
                    .name("Fluzzard")
                    .score(2)
                    .character("Kazuya"),
                SetType::Ft3,
            ),
            TestCase::new(
                "Joy C  Palu 5-3 Greninja/Incineroar nick",
                PlayerInfoForSetBuilder::default()
                    .name("Joy C")
                    .score(5)
                    .character("Palu"),
                PlayerInfoForSetBuilder::default()
                    .name("nick")
                    .score(3)
                    .character("Kazuya"),
                SetType::Ft5,
            ),
            // If one player's characters are not reported then treat the set as not having any chars.
            TestCase::new(
                "Karasu 5-0 Phish randoms",
                PlayerInfoForSetBuilder::default().name("Karasu").score(5),
                PlayerInfoForSetBuilder::default().name("Phish").score(3),
                SetType::Ft5,
            ),
            TestCase::new(
                "nick greninja 5 - 1 GnW, Sora Withering.Rxse<3",
                PlayerInfoForSetBuilder::default()
                    .name("nick")
                    .score(5)
                    .character("greninja"),
                PlayerInfoForSetBuilder::default()
                    .name("Withering.Rxse<3")
                    .score(1)
                    .character("Sora"),
                SetType::Ft5,
            ),
            TestCase::new(
                "@nick greninja 10-7 ganon @Emmie Katelyn",
                PlayerInfoForSetBuilder::default()
                    .name("nick")
                    .score(10)
                    .character("greninja"),
                PlayerInfoForSetBuilder::default()
                    .name("Emmie Katelyn")
                    .score(7)
                    .character("ganon"),
                SetType::Ft10,
            ),
            TestCase::new(
                "@Fluzzard Fox, Kazuya 2 - 5 Aegis @LYM? | PWR BRAIDEN 1# KIRBY FAN",
                PlayerInfoForSetBuilder::default()
                    .name("Fox, Kazuya")
                    .score(2)
                    .character("Fox, Kazuya"),
                PlayerInfoForSetBuilder::default()
                    .name("LYM? | PWR BRAIDEN 1# KIRBY FAN")
                    .score(5)
                    .character("Aegis"),
                SetType::Ft5,
            ),
        ];

        let parser = GameSetMessageParser::default();

        for test in tests {
            test.run_test(&parser);
        }
    }
}
