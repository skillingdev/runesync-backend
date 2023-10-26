use reqwest::StatusCode;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HiscoresUser {
    pub name: String,
    pub score: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HiscoresIndex {
    pub users: Vec<HiscoresUser>,
}

pub async fn hiscores_index(
    page: usize,
) -> Result<Option<HiscoresIndex>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let selector =
        Selector::parse("tr.personal-hiscores__row").map_err(|_| "failed selector parse")?;
    let href_selector = Selector::parse("a").map_err(|_| "failed selector parse")?;
    let score_selector = Selector::parse("td.right").map_err(|_| "failed selector parse")?;
    let next_selector = Selector::parse("a.personal-hiscores__pagination-arrow--down")
        .map_err(|_| "failed selector parse")?;

    let mut users: Vec<HiscoresUser> = Vec::new();

    let url = format!("https://secure.runescape.com/m=hiscore_oldschool_seasonal/overall?category_type=1&table=0&page={}", page);
    let response = reqwest::get(url).await?;
    let document = Html::parse_document(&response.text().await?);

    if let Some(next) = document.select(&next_selector).next() {
        let mut href = next
            .value()
            .attr("href")
            .ok_or("Can't get href")?
            .chars()
            .collect::<Vec<_>>();
        href.drain(0..37);
        let next_page: usize = href.into_iter().collect::<String>().parse()?;
        if next_page <= page {
            return Ok(None);
        }
    } else {
        return Ok(None);
    }

    for element in document.select(&selector) {
        if let Some(user) = element.select(&href_selector).next() {
            let mut scores = element.select(&score_selector);
            let _ = scores.next();
            if let Some(score) = scores.next() {
                users.push(HiscoresUser {
                    name: user.text().collect::<String>().replace('\u{A0}', " "),
                    score: score
                        .text()
                        .collect::<String>()
                        .trim()
                        .replace(",", "")
                        .parse()?,
                });
            }
        }
    }

    Ok(Some(HiscoresIndex { users }))
}

pub async fn user_hiscore(
    user: String,
) -> Result<Option<Hiscore>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let url = format!(
        "https://secure.runescape.com/m=hiscore_oldschool_seasonal/index_lite.ws?player={}",
        user
    );
    let res = reqwest::get(url).await?;
    if res.status() != StatusCode::OK {
        return Ok(None);
    }
    let response = res.text().await?;
    let entries: Vec<&str> = response.split('\n').collect();
    Ok(Some(Hiscore {
        skills: HiscoreSkills {
            overall: extract_skill_entry(entries[0])?,
            attack: extract_skill_entry(entries[1])?,
            defence: extract_skill_entry(entries[2])?,
            strength: extract_skill_entry(entries[3])?,
            hitpoints: extract_skill_entry(entries[4])?,
            ranged: extract_skill_entry(entries[5])?,
            prayer: extract_skill_entry(entries[6])?,
            magic: extract_skill_entry(entries[7])?,
            cooking: extract_skill_entry(entries[8])?,
            woodcutting: extract_skill_entry(entries[9])?,
            fletching: extract_skill_entry(entries[10])?,
            fishing: extract_skill_entry(entries[11])?,
            firemaking: extract_skill_entry(entries[12])?,
            crafting: extract_skill_entry(entries[13])?,
            smithing: extract_skill_entry(entries[14])?,
            mining: extract_skill_entry(entries[15])?,
            herblore: extract_skill_entry(entries[16])?,
            agility: extract_skill_entry(entries[17])?,
            thieving: extract_skill_entry(entries[18])?,
            slayer: extract_skill_entry(entries[19])?,
            farming: extract_skill_entry(entries[20])?,
            runecraft: extract_skill_entry(entries[21])?,
            hunter: extract_skill_entry(entries[22])?,
            construction: extract_skill_entry(entries[23])?,
        },
        activities: HiscoreActivities {
            league_points: extract_activity_entry(entries[24]),
            //    deadman_points: extract_activity_entry(entries[25]),
            //    bounty_hunter_-_hunter: extract_activity_entry(entries[26]),
            //    bounty_hunter_-_rogue: extract_activity_entry(entries[27]),
            //    bounty_hunter_(legacy)_-_hunter: extract_activity_entry(entries[28]),
            //    bounty_hunter_(legacy)_-_rogue: extract_activity_entry(entries[29]),
            clue_scrolls_all: extract_activity_entry(entries[30]),
            clue_scrolls_beginner: extract_activity_entry(entries[31]),
            clue_scrolls_easy: extract_activity_entry(entries[32]),
            clue_scrolls_medium: extract_activity_entry(entries[33]),
            clue_scrolls_hard: extract_activity_entry(entries[34]),
            clue_scrolls_elite: extract_activity_entry(entries[35]),
            clue_scrolls_master: extract_activity_entry(entries[36]),
            //    lms_-_rank: extract_activity_entry(entries[37]),
            //    pvp_arena_-_rank: extract_activity_entry(entries[38]),
            soul_wars_zeal: extract_activity_entry(entries[39]),
            rifts_closed: extract_activity_entry(entries[40]),
            abyssal_sire: extract_activity_entry(entries[41]),
            alchemical_hydra: extract_activity_entry(entries[42]),
            artio: extract_activity_entry(entries[43]),
            barrows_chests: extract_activity_entry(entries[44]),
            bryophyta: extract_activity_entry(entries[45]),
            callisto: extract_activity_entry(entries[46]),
            calvarion: extract_activity_entry(entries[47]),
            cerberus: extract_activity_entry(entries[48]),
            chambers_of_xeric: extract_activity_entry(entries[49]),
            chambers_of_xeric_challenge_mode: extract_activity_entry(entries[50]),
            chaos_elemental: extract_activity_entry(entries[51]),
            chaos_fanatic: extract_activity_entry(entries[52]),
            commander_zilyana: extract_activity_entry(entries[53]),
            corporeal_beast: extract_activity_entry(entries[54]),
            crazy_archaeologist: extract_activity_entry(entries[55]),
            dagannoth_prime: extract_activity_entry(entries[56]),
            dagannoth_rex: extract_activity_entry(entries[57]),
            dagannoth_supreme: extract_activity_entry(entries[58]),
            deranged_archaeologist: extract_activity_entry(entries[59]),
            duke_sucellus: extract_activity_entry(entries[60]),
            general_graardor: extract_activity_entry(entries[61]),
            giant_mole: extract_activity_entry(entries[62]),
            grotesque_guardians: extract_activity_entry(entries[63]),
            hespori: extract_activity_entry(entries[64]),
            kalphite_queen: extract_activity_entry(entries[65]),
            king_black_dragon: extract_activity_entry(entries[66]),
            kraken: extract_activity_entry(entries[67]),
            kreearra: extract_activity_entry(entries[68]),
            kril_tsutsaroth: extract_activity_entry(entries[69]),
            mimic: extract_activity_entry(entries[70]),
            nex: extract_activity_entry(entries[71]),
            nightmare: extract_activity_entry(entries[72]),
            phosanis_nightmare: extract_activity_entry(entries[73]),
            obor: extract_activity_entry(entries[74]),
            phantom_muspah: extract_activity_entry(entries[75]),
            sarachnis: extract_activity_entry(entries[76]),
            scorpia: extract_activity_entry(entries[77]),
            skotizo: extract_activity_entry(entries[78]),
            spindel: extract_activity_entry(entries[79]),
            tempoross: extract_activity_entry(entries[80]),
            the_gauntlet: extract_activity_entry(entries[81]),
            the_corrupted_gauntlet: extract_activity_entry(entries[82]),
            the_leviathan: extract_activity_entry(entries[83]),
            the_whisperer: extract_activity_entry(entries[84]),
            theatre_of_blood: extract_activity_entry(entries[85]),
            theatre_of_blood_hard_mode: extract_activity_entry(entries[86]),
            thermonuclear_smoke_devil: extract_activity_entry(entries[87]),
            tombs_of_amascut: extract_activity_entry(entries[88]),
            tombs_of_amascut_expert_mode: extract_activity_entry(entries[89]),
            tzkal_zuk: extract_activity_entry(entries[90]),
            tztok_jad: extract_activity_entry(entries[91]),
            vardorvis: extract_activity_entry(entries[92]),
            venenatis: extract_activity_entry(entries[93]),
            vetion: extract_activity_entry(entries[94]),
            vorkath: extract_activity_entry(entries[95]),
            wintertodt: extract_activity_entry(entries[96]),
            zalcano: extract_activity_entry(entries[97]),
            zulrah: extract_activity_entry(entries[98]),
        },
    }))
}

fn extract_skill_entry(
    entry: &str,
) -> Result<HiscoreSkillEntry, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let entries: Vec<u32> = entry
        .split(',')
        .map(|s| s.parse::<u32>().map_err(|_| "bad u32"))
        .collect::<Result<Vec<u32>, &str>>()?;
    Ok(HiscoreSkillEntry {
        rank: *entries.get(0).ok_or("err no 0 index")?,
        level: *entries.get(1).ok_or("err no 1 index")?,
        xp: *entries.get(2).ok_or("err no 2 index")?,
    })
}

fn extract_activity_entry(entry: &str) -> Option<HiscoreActivityEntry> {
    let entries: Vec<u32> = entry
        .split(',')
        .map(|s| s.parse::<u32>().ok())
        .collect::<Option<Vec<u32>>>()?;
    Some(HiscoreActivityEntry {
        rank: *entries.get(0)?,
        score: *entries.get(1)?,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HiscoreSkillEntry {
    xp: u32,
    level: u32,
    rank: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HiscoreSkills {
    overall: HiscoreSkillEntry,
    attack: HiscoreSkillEntry,
    defence: HiscoreSkillEntry,
    strength: HiscoreSkillEntry,
    hitpoints: HiscoreSkillEntry,
    ranged: HiscoreSkillEntry,
    prayer: HiscoreSkillEntry,
    magic: HiscoreSkillEntry,
    cooking: HiscoreSkillEntry,
    woodcutting: HiscoreSkillEntry,
    fletching: HiscoreSkillEntry,
    fishing: HiscoreSkillEntry,
    firemaking: HiscoreSkillEntry,
    crafting: HiscoreSkillEntry,
    smithing: HiscoreSkillEntry,
    mining: HiscoreSkillEntry,
    herblore: HiscoreSkillEntry,
    agility: HiscoreSkillEntry,
    thieving: HiscoreSkillEntry,
    slayer: HiscoreSkillEntry,
    farming: HiscoreSkillEntry,
    runecraft: HiscoreSkillEntry,
    hunter: HiscoreSkillEntry,
    construction: HiscoreSkillEntry,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HiscoreActivityEntry {
    score: u32,
    rank: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HiscoreActivities {
    league_points: Option<HiscoreActivityEntry>,
    //    deadman_points: Option<HiscoreActivityEntry>,
    //    bounty_hunter_-_hunter: Option<HiscoreActivityEntry>,
    //    bounty_hunter_-_rogue: Option<HiscoreActivityEntry>,
    //    bounty_hunter_(legacy)_-_hunter: Option<HiscoreActivityEntry>,
    //    bounty_hunter_(legacy)_-_rogue: Option<HiscoreActivityEntry>,
    clue_scrolls_all: Option<HiscoreActivityEntry>,
    clue_scrolls_beginner: Option<HiscoreActivityEntry>,
    clue_scrolls_easy: Option<HiscoreActivityEntry>,
    clue_scrolls_medium: Option<HiscoreActivityEntry>,
    clue_scrolls_hard: Option<HiscoreActivityEntry>,
    clue_scrolls_elite: Option<HiscoreActivityEntry>,
    clue_scrolls_master: Option<HiscoreActivityEntry>,
    //    lms_-_rank: Option<HiscoreActivityEntry>,
    //    pvp_arena_-_rank: Option<HiscoreActivityEntry>,
    soul_wars_zeal: Option<HiscoreActivityEntry>,
    rifts_closed: Option<HiscoreActivityEntry>,
    abyssal_sire: Option<HiscoreActivityEntry>,
    alchemical_hydra: Option<HiscoreActivityEntry>,
    artio: Option<HiscoreActivityEntry>,
    barrows_chests: Option<HiscoreActivityEntry>,
    bryophyta: Option<HiscoreActivityEntry>,
    callisto: Option<HiscoreActivityEntry>,
    calvarion: Option<HiscoreActivityEntry>,
    cerberus: Option<HiscoreActivityEntry>,
    chambers_of_xeric: Option<HiscoreActivityEntry>,
    chambers_of_xeric_challenge_mode: Option<HiscoreActivityEntry>,
    chaos_elemental: Option<HiscoreActivityEntry>,
    chaos_fanatic: Option<HiscoreActivityEntry>,
    commander_zilyana: Option<HiscoreActivityEntry>,
    corporeal_beast: Option<HiscoreActivityEntry>,
    crazy_archaeologist: Option<HiscoreActivityEntry>,
    dagannoth_prime: Option<HiscoreActivityEntry>,
    dagannoth_rex: Option<HiscoreActivityEntry>,
    dagannoth_supreme: Option<HiscoreActivityEntry>,
    deranged_archaeologist: Option<HiscoreActivityEntry>,
    duke_sucellus: Option<HiscoreActivityEntry>,
    general_graardor: Option<HiscoreActivityEntry>,
    giant_mole: Option<HiscoreActivityEntry>,
    grotesque_guardians: Option<HiscoreActivityEntry>,
    hespori: Option<HiscoreActivityEntry>,
    kalphite_queen: Option<HiscoreActivityEntry>,
    king_black_dragon: Option<HiscoreActivityEntry>,
    kraken: Option<HiscoreActivityEntry>,
    kreearra: Option<HiscoreActivityEntry>,
    kril_tsutsaroth: Option<HiscoreActivityEntry>,
    mimic: Option<HiscoreActivityEntry>,
    nex: Option<HiscoreActivityEntry>,
    nightmare: Option<HiscoreActivityEntry>,
    phosanis_nightmare: Option<HiscoreActivityEntry>,
    obor: Option<HiscoreActivityEntry>,
    phantom_muspah: Option<HiscoreActivityEntry>,
    sarachnis: Option<HiscoreActivityEntry>,
    scorpia: Option<HiscoreActivityEntry>,
    skotizo: Option<HiscoreActivityEntry>,
    spindel: Option<HiscoreActivityEntry>,
    tempoross: Option<HiscoreActivityEntry>,
    the_gauntlet: Option<HiscoreActivityEntry>,
    the_corrupted_gauntlet: Option<HiscoreActivityEntry>,
    the_leviathan: Option<HiscoreActivityEntry>,
    the_whisperer: Option<HiscoreActivityEntry>,
    theatre_of_blood: Option<HiscoreActivityEntry>,
    theatre_of_blood_hard_mode: Option<HiscoreActivityEntry>,
    thermonuclear_smoke_devil: Option<HiscoreActivityEntry>,
    tombs_of_amascut: Option<HiscoreActivityEntry>,
    tombs_of_amascut_expert_mode: Option<HiscoreActivityEntry>,
    tzkal_zuk: Option<HiscoreActivityEntry>,
    tztok_jad: Option<HiscoreActivityEntry>,
    vardorvis: Option<HiscoreActivityEntry>,
    venenatis: Option<HiscoreActivityEntry>,
    vetion: Option<HiscoreActivityEntry>,
    vorkath: Option<HiscoreActivityEntry>,
    wintertodt: Option<HiscoreActivityEntry>,
    zalcano: Option<HiscoreActivityEntry>,
    zulrah: Option<HiscoreActivityEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hiscore {
    skills: HiscoreSkills,
    activities: HiscoreActivities,
}
