use reals_server_bot_common::types::{
    DiscordUserIdentifier, GameSetData, PlayerInfoForSet, SetType,
};
use regex::Regex;

const REGEX_STR: &str = r"<@(?<p1_disc_id>\d+)> *(?<p1_char_str>.+)? (?<pi_score>\d)+ *- *(?<p2_score>\d+) +(?<p2_char_str>.+)? *<@(?<p2_disc_id>\d+)>";

#[derive(Debug)]
pub struct GameSetMessageParser {
    regex: Regex,
}

impl Default for GameSetMessageParser {
    fn default() -> Self {
        Self {
            regex: Regex::new(REGEX_STR).unwrap(),
        }
    }
}

impl GameSetMessageParser {
    pub fn extract_game_set_data_from_discord_msg_if_any(&self, msg: &str) -> Option<GameSetData> {
        self.regex.captures(msg).map(|captures| {
            let p1_disc_id = captures
                .name("p1_disc_id")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let p1_char_str = captures.name("p1_char_str").map(|s| s.as_str().to_string());
            let p1_score = captures.name("p1_score").unwrap().as_str().parse().unwrap();

            let p2_score = captures.name("p2_score").unwrap().as_str().parse().unwrap();
            let p2_char_str = captures.name("p2_char_str").map(|s| s.as_str().to_string());
            let p2_disc_id = captures
                .name("p2_disc_id")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            GameSetData {
                p1_info: PlayerInfoForSet {
                    user_identifier: DiscordUserIdentifier::Id(p1_disc_id),
                    score: p1_score,
                    character: p1_char_str,
                },
                p2_info: PlayerInfoForSet {
                    user_identifier: DiscordUserIdentifier::Id(p2_disc_id),
                    score: p2_score,
                    character: p2_char_str,
                },
                set_type: extract_set_type_from_scores(p1_score, p2_score),
            }
        })
    }
}

fn extract_set_type_from_scores(p1_score: usize, p2_score: usize) -> SetType {
    let max_score = p1_score.max(p2_score);

    match max_score {
        2 => SetType::Ft2,
        3 => SetType::Ft3,
        5 => SetType::Ft5,
        10 => SetType::Ft10,
        n => SetType::Ftn(n),
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
                panic!(
                    "Failure on {} (Expected: {:#?}, Got: {:#?})",
                    self.input, self.expected, parsed_res
                );
            }
        }
    }

    #[test]
    fn msg_parse_tests() {
        let tests = vec![
            TestCase::new(
                "Fluzzard Kazuya 5 - 0 Yoshi ./rust_man",
                PlayerInfoForSetBuilder::default()
                    .user_identifier("Fluzzard")
                    .score(5)
                    .character("Kazuya"),
                PlayerInfoForSetBuilder::default()
                    .user_identifier("./rust_man")
                    .score(0)
                    .character("Yoshi"),
                SetType::Ft5,
            ),
            TestCase::new(
                "./rust_man Yoshi 3 - 2 Kazuya Fluzzard",
                PlayerInfoForSetBuilder::default()
                    .user_identifier("./rust_man")
                    .score(3)
                    .character("Yoshi"),
                PlayerInfoForSetBuilder::default()
                    .user_identifier("Fluzzard")
                    .score(2)
                    .character("Kazuya"),
                SetType::Ft3,
            ),
            TestCase::new(
                "Joy C  Palu 5-3 Greninja/Incineroar nick",
                PlayerInfoForSetBuilder::default()
                    .user_identifier("Joy C")
                    .score(5)
                    .character("Palu"),
                PlayerInfoForSetBuilder::default()
                    .user_identifier("nick")
                    .score(3)
                    .character("Kazuya"),
                SetType::Ft5,
            ),
            // If one player's characters are not reported then treat the set as not having any chars.
            TestCase::new(
                "Karasu 5-0 Phish randoms",
                PlayerInfoForSetBuilder::default()
                    .user_identifier("Karasu")
                    .score(5),
                PlayerInfoForSetBuilder::default()
                    .user_identifier("Phish")
                    .score(3),
                SetType::Ft5,
            ),
            TestCase::new(
                "nick greninja 5 - 1 GnW, Sora Withering.Rxse<3",
                PlayerInfoForSetBuilder::default()
                    .user_identifier("nick")
                    .score(5)
                    .character("greninja"),
                PlayerInfoForSetBuilder::default()
                    .user_identifier("Withering.Rxse<3")
                    .score(1)
                    .character("Sora"),
                SetType::Ft5,
            ),
            TestCase::new(
                "@nick greninja 10-7 ganon @Emmie Katelyn",
                PlayerInfoForSetBuilder::default()
                    .user_identifier("nick")
                    .score(10)
                    .character("greninja"),
                PlayerInfoForSetBuilder::default()
                    .user_identifier("Emmie Katelyn")
                    .score(7)
                    .character("ganon"),
                SetType::Ft10,
            ),
            TestCase::new(
                "@Fluzzard Fox, Kazuya 2 - 5 Aegis @LYM? | PWR BRAIDEN 1# KIRBY FAN",
                PlayerInfoForSetBuilder::default()
                    .user_identifier("Fox, Kazuya")
                    .score(2)
                    .character("Fox, Kazuya"),
                PlayerInfoForSetBuilder::default()
                    .user_identifier("LYM? | PWR BRAIDEN 1# KIRBY FAN")
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
