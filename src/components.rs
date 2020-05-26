use crate::msg::{Msg, SetupState};
use crate::player::{Player, Rank};
use seed::{prelude::*, *};
use std::collections::HashMap;

pub fn header() -> Node<Msg> {
    header![
        class!["mt-4", "flex", "justify-center", "text-3xl", "font-bold"],
        h1![
            class!["pb-2", "border-b", "px-8", "border-black"],
            "Tycoon Scorer"
        ]
    ]
}

fn table_heading(text: &str) -> Node<Msg> {
    th![
        class![
            "px-4",
            "py-2",
            "font-bold",
            "pb-1",
            "border-b",
            "border-gray-600",
            "text-indigo-700",
            "text-2xl"
        ],
        text
    ]
}

fn rank_string(rank: &Option<Rank>) -> String {
    match rank {
        Some(x) => x.to_string(),
        None => "None".into(),
    }
}

fn player_row(id: &usize, player: &Player, already_out: bool) -> Node<Msg> {
    tr![
        class!["my-2"],
        td![
            class![
                "inline-block",
                "pl-4",
                "text-left",
                "font-bold",
                "w-full",
                "h-10"
            ],
            player.name.as_str(),
        ],
        td![class!["text-center", "h-8"], format!("{}", player.score),],
        td![
            class![
                "text-right",
                "inline-block",
                "pr-4",
                "w-full",
                "h-10",
                "text-center"
            ],
            rank_string(&player.rank)
        ],
        td![
            class!["h-10"],
            button![
                class! [
                   "px-2",
                   "py-1",
                   "rounded-sm",
                   "text-white",
                   "hover:shadow" => !already_out,
                   "bg-indigo-600" => !already_out,
                   "hover:bg-indigo-800" => !already_out,
                   "bg-gray-600" => already_out,
                ],
                attrs! {At::Disabled => already_out.as_at_value()},
                "Go Out",
                simple_ev(Ev::Click, Msg::GoOut(*id)),
            ]
        ]
    ]
}

pub fn score_table(players: &HashMap<usize, Player>, players_out: &HashMap<usize, Rank>) -> Node<Msg> {
    table![
        class!["table-auto", "border-collapse", "mt-2"],
        thead![tr![
            table_heading("Player Name"),
            table_heading("Score"),
            table_heading("Rank"),
            table_heading(""),
        ],],
        tbody![
            class!["px-4", "pt-2", "text-lg"],
            players
                .iter()
                .map(|(i, player)| player_row(i, player, players_out.contains_key(i))),
        ]
    ]
}

pub fn setup_mode(setup_state: &SetupState) -> Node<Msg> {
    div![
        class!["flex", "flex-col", "w-full", "max-w-2xl"],
        h1![class!["font-bold", "text-xl", "text-center"], "Add Players"],
        div![
            (0..setup_state.num_of_inputs).map(|i| {
                input![
                    class![
                        "bg-indigo-100",
                        "border-indigo-500",
                        "border",
                        "rounded",
                        "px-4",
                        "py-2",
                        "block",
                        "my-2",
                        "w-full",
                        "max-w-xl",
                        "mx-auto"
                    ],
                    attrs! {
                        At::Type => "text".to_string(),
                        At::Placeholder => format! ("Player {}", i + 1),
                        At::Value => setup_state.player_names.get(&i).unwrap_or(&"".to_string())
                    },
                    input_ev(Ev::Input, move |text| Msg::AddPlayer(text, i))
                ]
            }),
            div![
                class!["flex"],
                button![
                    class![
                        "px-4",
                        "py-2",
                        "bg-yellow-600",
                        "hover:shadow",
                        "hover:bg-yellow-800",
                        "text-white",
                        "rounded-full",
                        "mt-2",
                        "mx-auto"
                    ],
                    simple_ev(Ev::Click, Msg::SetupComplete),
                    "Cancel"
                ],
                button![
                    class![
                        "px-4",
                        "py-2",
                        "bg-indigo-600",
                        "hover:shadow",
                        "hover:bg-indigo-800",
                        "text-white",
                        "rounded-full",
                        "mt-2",
                        "mx-auto"
                    ],
                    simple_ev(Ev::Click, Msg::MorePlayers),
                    "Add Another"
                ],
                button![
                    class![
                        "px-4",
                        "py-2",
                        "bg-green-600",
                        "hover:shadow",
                        "hover:bg-green-800",
                        "text-white",
                        "rounded-full",
                        "mt-2",
                        "mx-auto"
                    ],
                    simple_ev(Ev::Click, Msg::SavePlayers),
                    "Save Players"
                ],
            ],
        ],
    ]
}

pub fn game_over_mode(ranking: Vec<Player>) -> Node<Msg> {
    div![
        class![
            "flex",
            "w-full",
            "items-center",
            "mt-2",
            "max-w-3xl",
            "mx-auto",
            "flex-col"
        ],
        section![
            span![
                class!["text-indigo-600", "font-bold", "text-2xl"],
                "WINNER: "
            ],
            span![class!["font-bold", "text-2xl"], ranking[0].name.as_str()],
        ],
        section![
            h4![
                class![
                    "font-bold",
                    "text-xl",
                    "text-indigo-600",
                    "w-full",
                    "text-center"
                ],
                "Game Summary",
            ],
            ranking.iter().map(|p| player_summary(p)),
        ]
    ]
}

fn player_summary(player: &Player) -> Node<Msg> {
    div![
        class![
            "my-2",
            "px-4",
            "py-2",
            "bg-indigo-100",
            "border-indigo-600",
            "flex",
            "flex-col",
            "shadow",
            "border",
            "rounded-sm"
        ],
        h4![
            class![
                "text-indigo-600",
                "font-bold",
                "text-lg",
                "text-center",
                "w-full"
            ],
            player.name.as_str()
        ],
        div![
            class!["w-full"],
            span![class!["text-indigo-600"], "Final Rank: "],
            match player.rank {
                Some(r) => r.to_string(),
                None => "Error".into(),
            }
        ],
        div![
            class!["w-full"],
            span![class!["text-indigo-600"], "Final Score: "],
            player.score.to_string(),
        ],
        div![
            class!["w-full"],
            span![class!["text-indigo-600"], "Past Ranks: "],
            format!(
                "{}, {}",
                player.past_ranks[0].to_string(),
                player.past_ranks[1].to_string()
            )
        ]
    ]
}
