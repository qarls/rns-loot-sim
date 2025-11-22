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

pub mod treasuresphere {
    use phf::{OrderedMap, OrderedSet};
    use phf_macros::{phf_ordered_map, phf_ordered_set};

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
    pub fn is_item_in_ts_pos(item: &usize, ts_i: &usize, ts_count: &usize) -> bool {
        let delta = ts_count - ts_i; // 1..=6
        match NOT_IN_LAST_SPHERES.get(&(*item as u32)) {
            //if 2 (topaz charm), then as long as delta is 1 or 2, it returns false
            Some(val) if val >= &delta => return false,
            Some(_) => return true,
            None => return true,
        };
    }

    impl Colors {
        pub fn items_in_ts(&self) -> Vec<usize> {
            //This needs to be modifiable
            match &self {
                Colors::Normal => (0..*super::IT_COUNT).collect(),
                Colors::Opal => IS_OPAL.iter().map(|x| *x).collect(),
                Colors::Sapphire => IS_SAPPHIRE.iter().map(|x| *x).collect(),
                Colors::Ruby => IS_RUBY.iter().map(|x| *x).collect(),
                Colors::Garnet => IS_GARNET.iter().map(|x| *x).collect(),
                Colors::Emerald => IS_EMERALD.iter().map(|x| *x).collect(),
            }
        }

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

    // 0..=23 | 120..=151 => true,
    pub static IS_OPAL: OrderedSet<usize> = phf_ordered_set! {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23,
        120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | 128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151,
    };

    // 24..=47 | 120..=127 | 152..=175 => true,
    pub static IS_SAPPHIRE: OrderedSet<usize> = phf_ordered_set! {
        24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47,
        120 | 121 | 122 | 123 | 124 | 125 | 126 | 127,
        152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 | 160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175,
    };

    // 48..=71 | 128..=135 | 152..=159 | 176..=191,
    pub static IS_RUBY: OrderedSet<usize> = phf_ordered_set! {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71,
        128 | 129 | 130 | 131 | 132 | 133 | 134 | 135,
        152 | 153 | 154 | 155 | 156 | 157 | 158 | 159,
        176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 | 189 | 190 | 191,
    };

    // 72..=95 | 136..=143 | 160..=167 | 176..=183 | 192..=199,
    pub static IS_GARNET: OrderedSet<usize> = phf_ordered_set! {
        72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95,
        136 | 137 | 138 | 139 | 140 | 141 | 142 | 143,
        160 | 161 | 162 | 163 | 164 | 165 | 166 | 167,
        176 | 177 | 178 | 179 | 180 | 181 | 182 | 183,
        192 | 193 | 194 | 195 | 196 | 197 | 198 | 199,
    };

    // 96..=119 | 144..=151 | 168..=175 | 184..=199,
    pub static IS_EMERALD: OrderedSet<usize> = phf_ordered_set! {
        96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119,
        144 | 145 | 146 | 147 | 148 | 149 | 150 | 151,
        168 | 169 | 170 | 171 | 172 | 173 | 174 | 175,
        184 | 185 | 186 | 187 | 188 | 189 | 190 | 191 | 192 | 193 | 194 | 195 | 196 | 197 | 198 | 199,
    };

    pub static NOT_IN_LAST_SPHERES: OrderedMap<u32, usize> = phf_ordered_map! {
        70 => 2, // topaz charm
        93 | 96 | 100 => 1,// silver coin, butterfly ocarina and blue rose
    };

    pub static ITEM_NAMES: OrderedSet<&'static str> = phf_ordered_set! {
        // ===Arcane set opal@[0..=7]===
        "it_raven_grimoire",
        "it_blackwing_staff",
        "it_curse_talon",
        "it_darkmagic_blade",
        "it_witchs_cloak",
        "it_crowfeather_hairpin",
        "it_redblack_ribbon",
        "it_opal_necklace",
        // ===Night set opal@[8..=15]===
        "it_sleeping_greatbow",
        "it_crescentmoon_dagger",
        "it_lullaby_harp",
        "it_nightstar_grimoire",
        "it_moon_pendant",
        "it_pajama_hat",
        "it_stuffed_rabbit",
        "it_nightingale_gown",
        // ===Timespace set opal@[16..=23]===
        "it_eternity_flute",
        "it_timewarp_wand",
        "it_chrome_shield",
        "it_clockwork_tome",
        "it_haste_boots",
        "it_timemage_cap",
        "it_starry_cloak",
        "it_gemini_necklace",
        // ===Wind set sapphire@[24..=31]===
        "it_hawkfeather_fan",
        "it_windbite_dagger",
        "it_pidgeon_bow",
        "it_shinsoku_katana",
        "it_eaglewing_charm",
        "it_sparrow_feather",
        "it_winged_cap",
        "it_thiefs_coat",
        // ===Bloodwolf set sapphire@[32..=39]===
        "it_vampiric_dagger",
        "it_bloody_bandage",
        "it_leech_staff",
        "it_bloodhound_greatsword",
        "it_reaper_cloak",
        "it_bloodflower_brooch",
        "it_wolf_hood",
        "it_blood_vial",
        // ===Assassin set sapphire@[40..=47]===
        "it_black_wakizashi",
        "it_throwing_dagger",
        "it_assassins_knife",
        "it_ninjutsu_scroll",
        "it_shadow_bracelet",
        "it_ninja_robe",
        "it_kunoichi_hood",
        "it_shinobi_tabi",
        // ===Rockdragon set ruby@[48..=55]===
        "it_dragonhead_spear",
        "it_granite_greatsword",
        "it_greysteel_shield",
        "it_stonebreaker_staff",
        "it_tough_gauntlet",
        "it_rockdragon_mail",
        "it_obsidian_hairpin",
        "it_iron_grieves",
        // ===Flame set ruby@[56..=63]===
        "it_volcano_spear",
        "it_reddragon_blade",
        "it_flame_bow",
        "it_meteor_staff",
        "it_phoenix_charm",
        "it_firescale_corset",
        "it_demon_horns",
        "it_flamewalker_boots",
        // ===Gem set ruby@[64..=71]===
        "it_diamond_shield",
        "it_peridot_rapier",
        "it_garnet_staff",
        "it_sapphire_violin",
        "it_emerald_chestplate",
        "it_amethyst_bracelet",
        "it_topaz_charm",
        "it_ruby_circlet",
        // ===Lightning set garnet@[72..=79]===
        "it_brightstorm_spear",
        "it_bolt_staff",
        "it_lightning_bow",
        "it_darkstorm_knife",
        "it_darkcloud_necklace",
        "it_crown_of_storms",
        "it_thunderclap_gloves",
        "it_storm_petticoat",
        // ===Shrine set garnet@[80..=87]===
        "it_holy_greatsword",
        "it_sacred_bow",
        "it_purification_rod",
        "it_ornamental_bell",
        "it_shrinemaidens_kosode",
        "it_redwhite_ribbon",
        "it_divine_mirror",
        "it_golden_chime",
        // ===Lucky set garnet@[88..=95]===
        "it_book_of_cheats",
        "it_golden_katana",
        "it_glittering_trumpet",
        "it_royal_staff",
        "it_ballroom_gown",
        "it_silver_coin",
        "it_queens_crown",
        "it_mimick_rabbitfoot",
        // ===Life set emerald@[96..=103]===
        "it_butterfly_ocarina",
        "it_fairy_spear",
        "it_moss_shield",
        "it_floral_bow",
        "it_blue_rose",
        "it_sunflower_crown",
        "it_midsummer_dress",
        "it_grasswoven_bracelet",
        // ===Poison set emerald@[104..=111]===
        "it_snakefang_dagger",
        "it_ivy_staff",
        "it_deathcap_tome",
        "it_spiderbite_bow",
        "it_compound_gloves",
        "it_poisonfrog_charm",
        "it_venom_hood",
        "it_chemists_coat",
        // ===Depth set emerald@[112..=119]===
        "it_seashell_shield",
        "it_necronomicon",
        "it_tidal_greatsword",
        "it_occult_dagger",
        "it_mermaid_scale",
        "it_hydrous_blob",
        "it_abyss_artifact",
        "it_lost_pendant",
        // ===Darkbite set OS@[120..=127]===
        "it_sawtooth_cleaver",
        "it_ravens_dagger",
        "it_killing_note",
        "it_blacksteel_buckler",
        "it_nightguard_gloves",
        "it_snipers_eyeglasses",
        "it_darkmage_charm",
        "it_firststrike_bracelet",
        // ===Timegem set OR@[128..=135]===
        "it_obsidian_rod",
        "it_darkglass_spear",
        "it_timespace_dagger",
        "it_quartz_shield",
        "it_pocketwatch",
        "it_nova_crown",
        "it_blackhole_charm",
        "it_twinstar_earrings",
        // ===Youkai set OG@[136..=143]===
        "it_kyou_no_omikuji",
        "it_youkai_bracelet",
        "it_oni_staff",
        "it_kappa_shield",
        "it_usagi_kamen",
        "it_red_tanzaku",
        "it_vega_spear",
        "it_altair_dagger",
        // ===Haunted set OE@[144..=151]===
        "it_ghost_spear",
        "it_phantom_dagger",
        "it_cursed_candlestaff",
        "it_smoke_shield",
        "it_haunted_gloves",
        "it_old_bonnet",
        "it_maid_outfit",
        "it_calling_bell",
        // ===Gladiator set SR@[152..=159]===
        "it_grandmaster_spear",
        "it_teacher_knife",
        "it_tactician_rod",
        "it_spiked_shield",
        "it_battlemaiden_armor",
        "it_gladiator_helmet",
        "it_lancer_gauntlets",
        "it_lion_charm",
        // ===Sparkblade set SG@[160..=167]===
        "it_bluebolt_staff",
        "it_lapis_sword",
        "it_shockwave_tome",
        "it_battery_shield",
        "it_raiju_crown",
        "it_staticshock_earrings",
        "it_stormdance_gown",
        "it_blackbolt_ribbon",
        // ===Swiftflight set SE@[168..=175]===
        "it_crane_katana",
        "it_falconfeather_dagger",
        "it_tornado_staff",
        "it_cloud_guard",
        "it_hermes_bow",
        "it_talon_charm",
        "it_tiny_wings",
        "it_feathered_overcoat",
        // ===Sacredflame set RG@[176..=183]===
        "it_sandpriestess_spear",
        "it_flamedancer_dagger",
        "it_whiteflame_staff",
        "it_sacred_shield",
        "it_marble_clasp",
        "it_sun_pendant",
        "it_tiny_hourglass",
        "it_desert_earrings",
        // ===Ruins set RE@[184..=191]===
        "it_giant_stone_club",
        "it_ruins_sword",
        "it_mountain_staff",
        "it_boulder_shield",
        "it_golems_claymore",
        "it_stoneplate_armor",
        "it_sacredstone_charm",
        "it_clay_rabbit",
        // ===Lakeshrine set GE@[192..=199]===
        "it_waterfall_polearm",
        "it_vorpal_dao",
        "it_jade_staff",
        "it_reflection_shield",
        "it_butterfly_hairpin",
        "it_watermage_pendant",
        "it_raindrop_earrings",
        "it_aquamarine_bracelet",
    };
}
