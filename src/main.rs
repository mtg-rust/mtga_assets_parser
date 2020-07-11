use std::io;
use serde::{Deserialize, Serialize};

extern crate mtga_resources_locator;
use mtga_resources_locator::assets_data_dir;

mod client;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct Card {
    title_id: i32,
    cmc: i32
}

#[derive(Debug)]
struct MagicCard {
    title: String,
    cmc: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Localization {
    iso_code: String,
    keys: Vec<LocalizationKey>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LocalizationKey {
    id: i32,
    text: String
}


fn get_card_by_id(id: i32, c: &str, l: &str) -> MagicCard {
    let cards: Vec<Card> = serde_json::from_str(c).unwrap();
    let loc: Vec<Localization> = serde_json::from_str(l).unwrap();

    let card_index = cards.iter()
        .position(|map| map.title_id == id)
        .expect("Can't find card with given ID");

    let card = cards[card_index];

    let title_key = loc[0].keys.iter()
        .position(|map| map.id == card.title_id)
        .expect("Can't find string translation");

    let title = &loc[0].keys[title_key].text;

    return MagicCard {
        title: title.to_string(),
        cmc: card.cmc,
    }
}



fn main() -> io::Result<()> {

    // Get assets data directory
    let data_dir = assets_data_dir().unwrap().unwrap();

    // Retrives Magic Client Data
    // let client = &mut client::ClientData::new(data_dir);
    let client = &mut client::ClientData::new(data_dir);
    let data = client.collect()?;

    let c = get_card_by_id(653,  &data.cards, &data.localization);
    println!("{:?}", c);

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_collects_data() {

        // Get assets data directory
        let data_dir = assets_data_dir().unwrap().unwrap();

        // Retrives Magic Client Data
        println!("{:?}", data_dir);
        let client = &mut client::ClientData::new(data_dir);
        let data = client.collect().expect("Could not return data.");

        assert_eq!(!data.cards.is_empty(), true);
        assert_eq!(!data.localization.is_empty(), true);
        assert_eq!(!data.enums.is_empty(), true);
        assert_eq!(!data.abilities.is_empty(), true);
    }
}

