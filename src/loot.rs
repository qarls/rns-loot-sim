// Module for game constants, hashmaps and helper functions associated with them

/// Treasuresphere Count
///
/// May break if changed from 6 as of right now.
pub static TS_COUNT: &'static usize = &6usize;

/// Item Count in game
pub static IT_COUNT: &'static usize = &200usize;

/// Max items found per Treasuresphere
pub static IT_FOUND_MAX_PER_TS: &'static usize = &5usize;

/// Module to call constants based on player count
pub mod player_loot {
    use anyhow::{bail, Error};

    pub fn loot_counts(player_count: usize) -> Result<Vec<usize>, Error> {
        let loot_counts = match player_count {
            1 => *ONE,
            2 => *TWO,
            3 => *THREE,
            4 => *FOUR,
            _ => bail!(
                "Invalid player count: {}\nPlease enter a number from 1 to 4.",
                player_count
            ),
        };
        Ok(loot_counts.to_vec())
    }

    pub fn loot_sum(player_count: usize) -> Result<usize, Error> {
        Ok(loot_counts(player_count)?.into_iter().sum())
    }

    static ONE: &'static [usize; 6] = &[5, 5, 3, 3, 3, 3];
    static TWO: &'static [usize; 6] = &[5, 5, 4, 4, 4, 4];
    static THREE: &'static [usize; 6] = &[5, 5, 4, 4, 4, 4];
    static FOUR: &'static [usize; 6] = &[5, 5, 5, 5, 5, 5];
}

// I realize I don't need to include "false", given retrieving these values
// will generate a none if not found, but ehh let's go this way.

pub mod treasuresphere {
    // #[allow(unused_imports)]
    use phf::OrderedMap;
    use phf_macros::phf_ordered_map;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Colors {
        Normal, // Reminder that you can find Normal 3 times
        Opal,
        Sapphire,
        Ruby,
        Garnet,
        Emerald,
    }

    /// Checks if the item is valid in the current Treasuresphere position
    pub fn is_item_in_ts_pos(item: &u32, ts_i: &usize, ts_count: &usize) -> bool {
        let delta = (ts_count - ts_i) as u32; // 1..=6
        match NOT_IN_LAST_SPHERES.get(&item) {
            //if 2 (topaz charm), then as long as delta is 1 or 2, it returns false
            Some(val) if val >= &delta => return false,
            Some(_) => return true,
            None => return true,
        };
    }

    impl Colors {
        /// Involves weighted indices, for use when generating treasurespheres
        pub fn from_index(index: u8) -> Self {
            match index {
                0 | 1 | 2 => Colors::Normal,
                3 => Colors::Opal,
                4 => Colors::Sapphire,
                5 => Colors::Ruby,
                6 => Colors::Garnet,
                7 => Colors::Emerald,
                _ => panic!("Unexpected treasuresphere index: {}", index),
            }
        }

        pub fn is_item_in_ts(&self, loot: &u32) -> bool {
            let option = match &self {
                Colors::Normal if *loot < *super::IT_COUNT as u32 => return true,
                Colors::Normal => panic!("Loot index is out of bounds: {}", loot),
                Colors::Opal => IS_OPAL.get(loot).clone(),
                Colors::Sapphire => IS_SAPPHIRE.get(loot).clone(),
                Colors::Ruby => IS_RUBY.get(loot).clone(),
                Colors::Garnet => IS_GARNET.get(loot).clone(),
                Colors::Emerald => IS_EMERALD.get(loot).clone(),
            };

            match option {
                Some(val) => return *val,
                None => panic!(
                    "Loot not found at index: {}\nTreasuresphere:{}\nfunction: is_item_in_ts()",
                    loot,
                    self.to_string()
                ),
            }
        }
    }

    // ToString feels so Rusty :D
    impl ToString for Colors {
        fn to_string(&self) -> String {
            match self {
                Colors::Normal => "normal",
                Colors::Opal => "opal",
                Colors::Sapphire => "sapphire",
                Colors::Ruby => "ruby",
                Colors::Garnet => "garnet",
                Colors::Emerald => "emerald",
            }
            .to_string()
        }
    }

    // I made an amatuer script from transforming it from format
    // `0..=23 | 120..=151 => true,`
    //
    // Reminder to self, don't use usize/isize (u64/i64) as keys.
    // They were the source of my issues with these returning None.
    // 0..=23 | 120..=151 => true,
    pub static IS_OPAL: OrderedMap<u32, bool> = phf_ordered_map! {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => true,
        24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 => false,
        120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | 128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 => true,
        152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 | 160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 | 176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 | 189 | 190 | 191 | 192 | 193 | 194 | 195 | 196 | 197 | 198 | 199 => false,
    };

    // 24..=47 | 120..=127 | 152..=175 => true,
    pub static IS_SAPPHIRE: OrderedMap<u32, bool> = phf_ordered_map! {
        24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 => true,
        120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 => true,
        152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 | 160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 => true,
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => false,
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 => false,
        128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 => false,
        176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 | 189 | 190 | 191 | 192 | 193 | 194 | 195 | 196 | 197 | 198 | 199 => false,
    };

    // 48..=71 | 128..=135 | 152..=159 | 176..=191 => true,
    pub static IS_RUBY: OrderedMap<u32, bool> = phf_ordered_map! {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 => true,
        128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 => true,
        152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 => true,
        176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 | 189 | 190 | 191 => true,
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 => false,
        72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 => false,
        136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 => false,
        160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 => false,
        192 | 193 | 194 | 195 | 196 | 197 | 198 | 199 => false,
    };

    // 72..=95 | 136..=143 | 160..=167 | 176..=183 | 192..=199 => true,
    pub static IS_GARNET: OrderedMap<u32, bool> = phf_ordered_map! {
        72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 => true,
        136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 => true,
        160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 => true,
        176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 => true,
        192 | 193 | 194 | 195 | 196 | 197 | 198 | 199 => true,
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 => false,
        96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | 128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 => false,
        144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 | 152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 => false,
        168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 => false,
        184 | 185 | 186 | 187 | 188 | 189 | 190 | 191 => false,
    };

    // 96..=119 | 144..=151 | 168..=175 | 184..=199 => true,
    pub static IS_EMERALD: OrderedMap<u32, bool> = phf_ordered_map! {
        96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 => true,
        144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 => true,
        168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 => true,
        184 | 185 | 186 | 187 | 188 | 189 | 190 | 191 | 192 | 193 | 194 | 195 | 196 | 197 | 198 | 199 => true,
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 => false,
        120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | 128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 => false,
        152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 | 160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 => false,
        176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 => false,
    };

    pub static NOT_IN_LAST_SPHERES: OrderedMap<u32, u32> = phf_ordered_map! {
        70 => 2, // topaz charm
        93 | 96 | 100 => 1,// silver coin, butterfly ocarina and blue rose
    };

    pub static ITEM_NAMES: OrderedMap<u32, &'static str> = phf_ordered_map! {
        // ===Arcane set opal===
        0	=> "it_raven_grimoire",
        1	=> "it_blackwing_staff",
        2	=> "it_curse_talon",
        3	=> "it_darkmagic_blade",
        4	=> "it_witchs_cloak",
        5	=> "it_crowfeather_hairpin",
        6	=> "it_redblack_ribbon",
        7	=> "it_opal_necklace",
        // ===Night set opal===
        8	=> "it_sleeping_greatbow",
        9	=> "it_crescentmoon_dagger",
        10	=> "it_lullaby_harp",
        11	=> "it_nightstar_grimoire",
        12	=> "it_moon_pendant",
        13	=> "it_pajama_hat",
        14	=> "it_stuffed_rabbit",
        15	=> "it_nightingale_gown",
        // ===Timespace set opal===
        16	=> "it_eternity_flute",
        17	=> "it_timewarp_wand",
        18	=> "it_chrome_shield",
        19	=> "it_clockwork_tome",
        20	=> "it_haste_boots",
        21	=> "it_timemage_cap",
        22	=> "it_starry_cloak",
        23	=> "it_gemini_necklace",
        // ===Wind set sapphire===
        24	=> "it_hawkfeather_fan",
        25	=> "it_windbite_dagger",
        26	=> "it_pidgeon_bow",
        27	=> "it_shinsoku_katana",
        28	=> "it_eaglewing_charm",
        29	=> "it_sparrow_feather",
        30	=> "it_winged_cap",
        31	=> "it_thiefs_coat",
        // ===Bloodwolf set sapphire===
        32	=> "it_vampiric_dagger",
        33	=> "it_bloody_bandage",
        34	=> "it_leech_staff",
        35	=> "it_bloodhound_greatsword",
        36	=> "it_reaper_cloak",
        37	=> "it_bloodflower_brooch",
        38	=> "it_wolf_hood",
        39	=> "it_blood_vial",
        // ===Assassin set sapphire===
        40	=> "it_black_wakizashi",
        41	=> "it_throwing_dagger",
        42	=> "it_assassins_knife",
        43	=> "it_ninjutsu_scroll",
        44	=> "it_shadow_bracelet",
        45	=> "it_ninja_robe",
        46	=> "it_kunoichi_hood",
        47	=> "it_shinobi_tabi",
        // ===Rockdragon set ruby===
        48	=> "it_dragonhead_spear",
        49	=> "it_granite_greatsword",
        50	=> "it_greysteel_shield",
        51	=> "it_stonebreaker_staff",
        52	=> "it_tough_gauntlet",
        53	=> "it_rockdragon_mail",
        54	=> "it_obsidian_hairpin",
        55	=> "it_iron_grieves",
        // ===Flame set ruby===
        56	=> "it_volcano_spear",
        57	=> "it_reddragon_blade",
        58	=> "it_flame_bow",
        59	=> "it_meteor_staff",
        60	=> "it_phoenix_charm",
        61	=> "it_firescale_corset",
        62	=> "it_demon_horns",
        63	=> "it_flamewalker_boots",
        // ===Gem set ruby===
        64	=> "it_diamond_shield",
        65	=> "it_peridot_rapier",
        66	=> "it_garnet_staff",
        67	=> "it_sapphire_violin",
        68	=> "it_emerald_chestplate",
        69	=> "it_amethyst_bracelet",
        70	=> "it_topaz_charm",
        71	=> "it_ruby_circlet",
        // ===Lightning set garnet===
        72	=> "it_brightstorm_spear",
        73	=> "it_bolt_staff",
        74	=> "it_lightning_bow",
        75	=> "it_darkstorm_knife",
        76	=> "it_darkcloud_necklace",
        77	=> "it_crown_of_storms",
        78	=> "it_thunderclap_gloves",
        79	=> "it_storm_petticoat",
        // ===Shrine set garnet===
        80	=> "it_holy_greatsword",
        81	=> "it_sacred_bow",
        82	=> "it_purification_rod",
        83	=> "it_ornamental_bell",
        84	=> "it_shrinemaidens_kosode",
        85	=> "it_redwhite_ribbon",
        86	=> "it_divine_mirror",
        87	=> "it_golden_chime",
        // ===Lucky set garnet===
        88	=> "it_book_of_cheats",
        89	=> "it_golden_katana",
        90	=> "it_glittering_trumpet",
        91	=> "it_royal_staff",
        92	=> "it_ballroom_gown",
        93	=> "it_silver_coin",
        94	=> "it_queens_crown",
        95	=> "it_mimick_rabbitfoot",
        // ===Life set emerald===
        96	=> "it_butterfly_ocarina",
        97	=> "it_fairy_spear",
        98	=> "it_moss_shield",
        99	=> "it_floral_bow",
        100	=> "it_blue_rose",
        101	=> "it_sunflower_crown",
        102	=> "it_midsummer_dress",
        103	=> "it_grasswoven_bracelet",
        // ===Poison set emerald===
        104	=> "it_snakefang_dagger",
        105	=> "it_ivy_staff",
        106	=> "it_deathcap_tome",
        107	=> "it_spiderbite_bow",
        108	=> "it_compound_gloves",
        109	=> "it_poisonfrog_charm",
        110	=> "it_venom_hood",
        111	=> "it_chemists_coat",
        // ===Depth set emerald===
        112	=> "it_seashell_shield",
        113	=> "it_necronomicon",
        114	=> "it_tidal_greatsword",
        115	=> "it_occult_dagger",
        116	=> "it_mermaid_scale",
        117	=> "it_hydrous_blob",
        118	=> "it_abyss_artifact",
        119	=> "it_lost_pendant",
        // ===Darkbite set OS===
        120	=> "it_sawtooth_cleaver",
        121	=> "it_ravens_dagger",
        122	=> "it_killing_note",
        123	=> "it_blacksteel_buckler",
        124	=> "it_nightguard_gloves",
        125	=> "it_snipers_eyeglasses",
        126	=> "it_darkmage_charm",
        127	=> "it_firststrike_bracelet",
        // ===Timegem set OR===
        128	=> "it_obsidian_rod",
        129	=> "it_darkglass_spear",
        130	=> "it_timespace_dagger",
        131	=> "it_quartz_shield",
        132	=> "it_pocketwatch",
        133	=> "it_nova_crown",
        134	=> "it_blackhole_charm",
        135	=> "it_twinstar_earrings",
        // ===Youkai set OG===
        136	=> "it_kyou_no_omikuji",
        137	=> "it_youkai_bracelet",
        138	=> "it_oni_staff",
        139	=> "it_kappa_shield",
        140	=> "it_usagi_kamen",
        141	=> "it_red_tanzaku",
        142	=> "it_vega_spear",
        143	=> "it_altair_dagger",
        // ===Haunted set OE===
        144	=> "it_ghost_spear",
        145	=> "it_phantom_dagger",
        146	=> "it_cursed_candlestaff",
        147	=> "it_smoke_shield",
        148	=> "it_haunted_gloves",
        149	=> "it_old_bonnet",
        150	=> "it_maid_outfit",
        151	=> "it_calling_bell",
        // ===Gladiator set SR===
        152	=> "it_grandmaster_spear",
        153	=> "it_teacher_knife",
        154	=> "it_tactician_rod",
        155	=> "it_spiked_shield",
        156	=> "it_battlemaiden_armor",
        157	=> "it_gladiator_helmet",
        158	=> "it_lancer_gauntlets",
        159	=> "it_lion_charm",
        // ===Sparkblade set SG===
        160	=> "it_bluebolt_staff",
        161	=> "it_lapis_sword",
        162	=> "it_shockwave_tome",
        163	=> "it_battery_shield",
        164	=> "it_raiju_crown",
        165	=> "it_staticshock_earrings",
        166	=> "it_stormdance_gown",
        167	=> "it_blackbolt_ribbon",
        // ===Swiftflight set SE===
        168	=> "it_crane_katana",
        169	=> "it_falconfeather_dagger",
        170	=> "it_tornado_staff",
        171	=> "it_cloud_guard",
        172	=> "it_hermes_bow",
        173	=> "it_talon_charm",
        174	=> "it_tiny_wings",
        175	=> "it_feathered_overcoat",
        // ===Sacredflame set RG===
        176	=> "it_sandpriestess_spear",
        177	=> "it_flamedancer_dagger",
        178	=> "it_whiteflame_staff",
        179	=> "it_sacred_shield",
        180	=> "it_marble_clasp",
        181	=> "it_sun_pendant",
        182	=> "it_tiny_hourglass",
        183	=> "it_desert_earrings",
        // ===Ruins set RE===
        184	=> "it_giant_stone_club",
        185	=> "it_ruins_sword",
        186	=> "it_mountain_staff",
        187	=> "it_boulder_shield",
        188	=> "it_golems_claymore",
        189	=> "it_stoneplate_armor",
        190	=> "it_sacredstone_charm",
        191	=> "it_clay_rabbit",
        // ===Lakeshrine set GE===
        192	=> "it_waterfall_polearm",
        193	=> "it_vorpal_dao",
        194	=> "it_jade_staff",
        195	=> "it_reflection_shield",
        196	=> "it_butterfly_hairpin",
        197	=> "it_watermage_pendant",
        198	=> "it_raindrop_earrings",
        199	=> "it_aquamarine_bracelet",
    };
}
