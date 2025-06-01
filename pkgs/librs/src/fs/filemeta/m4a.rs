use mp4ameta::{AdvisoryRating, ChannelConfig, MediaType, SampleRate};
use std::{collections::HashMap, path::PathBuf, time::Duration};

// Things I haven't covered here:
// - images
// - artworks
// - chapters

const DEFAULT_ADVISORY_RATING: &'static str = "Unknown Advisory";
const DEFAULT_ALBUM: &'static str = "Unknown Album";
const DEFAULT_ALBUM_ARTIST: &'static str = "Unknown Album Artist";
const DEFAULT_ARTIST: &'static str = "Unknown Artists";
const DEFAULT_AVG_BITRATE: &'static str = "";
const DEFAULT_BPM: &'static str = "";
const DEFAULT_CATEGORY: &'static str = "Unknown Category";
const DEFAULT_CHANNEL_CONFIG: &'static str = "Unknown Channel Config";
const DEFAULT_COMMENT: &'static str = "No Comment";
const DEFAULT_COMPOSER: &'static str = "Unknown Composer";
const DEFAULT_COPYRIGHT: &'static str = "Unknown Copyright";
const DEFAULT_CUSTOM_GENRE: &'static str = "No Custom Genre";
const DEFAULT_DESCRIPTION: &'static str = "No Description";
const DEFAULT_DISC_NUMBER: &'static str = "";
const DEFAULT_ENCODER: &'static str = "No Encoder";
const DEFAULT_GENRE: &'static str = "No Genre";
const DEFAULT_GROUPING: &'static str = "No Grouping";
const DEFAULT_ISRC: &'static str = "No ISRC";
const DEFAULT_KEYWORD: &'static str = "No Keyword";
const DEFAULT_LYRICS: &'static str = "No Lyrics";
const DEFAULT_LYRICIST: &'static str = "No Lyricist";
const DEFAULT_MAX_BITRATE: &'static str = "";
const DEFAULT_MEDIA_TYPE: &'static str = "Unset Media Type";
const DEFAULT_MOVEMENT: &'static str = "Unknown Movement";
const DEFAULT_MOVEMENT_COUNT: &'static str = "";
const DEFAULT_MOVEMENT_INDEX: &'static str = "";
const DEFAULT_SAMPLE_RATE: &'static str = "Unknown Sample Rate";
const DEFAULT_STANDARD_GENRE: &'static str = "0";
const DEFAULT_TITLE: &'static str = "Unknown Title";
const DEFAULT_TOTAL_DISCS: &'static str = "1";
const DEFAULT_TOTAL_TRACKS: &'static str = "1";
const DEFAULT_TRACK_NUMBER: &'static str = "1";
const DEFAULT_TV_EPISODE: &'static str = "0";
const DEFAULT_TV_EPISODE_NAME: &'static str = "Unknown Episode Name";
const DEFAULT_TV_NETWORK_NAME: &'static str = "Unknown Network";
const DEFAULT_TV_SEASON: &'static str = "0";
const DEFAULT_TV_SHOW_NAME: &'static str = "Unknown Show Name";
const DEFAULT_TV_SHOW_NAME_SORT_ORDER: &'static str = "Unknown Sort Order";
const DEFAULT_WORK: &'static str = "Unknown Work";
const DEFAULT_YEAR: &'static str = "Unknown Year";

pub struct Mp4a {
    /// The advisory rating (`rtng`).
    advisory_rating: Option<AdvisoryRating>,
    /// The album name (`©alb`).
    album: Option<String>,
    album_artist: Option<String>,
    album_artists: Vec<String>,
    artist: Option<String>,
    artists: Vec<String>,
    // artwork
    // artworks
    avg_bitrate: Option<u32>,
    bpm: Option<u16>,
    categories: Vec<String>,
    category: Option<String>,
    channel_config: Option<ChannelConfig>,
    comment: Option<String>,
    comments: Vec<String>,
    compilation: bool,
    composer: Option<String>,
    composers: Vec<String>,
    copyright: Option<String>,
    custom_genre: Option<String>,
    custom_genres: Vec<String>,
    description: Option<String>,
    descriptions: Vec<String>,
    disc_number: Option<u16>,
    duration: Duration,
    encoder: Option<String>,
    filetype: String,
    gapless_playback: bool,
    genre: Option<String>,
    genres: Vec<String>,
    grouping: Option<String>,
    groupings: Vec<String>,
    // images
    isrc: Option<String>,
    keyword: Option<String>,
    keywords: Vec<String>,
    lyrics: Option<String>,
    lyricist: Option<String>,
    lyricists: Vec<String>,
    max_bitrate: Option<u32>,
    /// The media type (`stik`).
    media_type: Option<MediaType>,
    movement: Option<String>,
    movement_count: Option<u16>,
    movement_index: Option<u16>,
    sample_rate: Option<SampleRate>,
    standard_genre: Option<u16>,
    standard_genres: Vec<u16>,
    title: Option<String>,
    total_discs: Option<u16>,
    total_tracks: Option<u16>,
    track: (Option<u16>, Option<u16>),
    track_number: Option<u16>,
    tv_episode: Option<u32>,
    tv_episode_name: Option<String>,
    tv_network_name: Option<String>,
    tv_season: Option<u32>,
    tv_show_name: Option<String>,
    tv_show_name_sort_order: Option<String>,
    work: Option<String>,
    year: Option<String>,
}

impl Mp4a {
    pub fn from_filepath(filepath: &PathBuf) -> Option<Self> {
        let tag = match mp4ameta::Tag::read_from_path(filepath) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };

        let album_artists = tag
            .album_artists()
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();
        let artists = tag.artists().map(|a| a.to_owned()).collect::<Vec<String>>();
        let categories = tag
            .categories()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|c| c.to_owned())
            .collect::<Vec<String>>();
        let comments = tag
            .comments()
            .map(|c| c.to_owned())
            .collect::<Vec<String>>();
        let composers = tag
            .composers()
            .map(|c| c.to_owned())
            .collect::<Vec<String>>();
        let custom_genres = tag
            .custom_genres()
            .map(|c| c.to_owned())
            .collect::<Vec<String>>();
        let descriptions = tag
            .descriptions()
            .map(|d| d.to_owned())
            .collect::<Vec<String>>();
        let genres = tag.genres().map(|g| g.to_owned()).collect::<Vec<String>>();
        let groupings = tag
            .groupings()
            .map(|g| g.to_owned())
            .collect::<Vec<String>>();
        let keywords = tag
            .keywords()
            .map(|k| k.to_owned())
            .collect::<Vec<String>>();
        let lyricists = tag
            .lyricists()
            .map(|l| l.to_owned())
            .collect::<Vec<String>>();
        let standard_genres = tag.standard_genres().collect::<Vec<u16>>();

        Some(Self {
            advisory_rating: tag.advisory_rating(),
            album: tag.album().map(|a| a.to_owned()),
            album_artist: tag.album_artist().map(|a| a.to_owned()),
            album_artists,
            artist: tag.artist().map(|a| a.to_owned()),
            artists,
            avg_bitrate: tag.avg_bitrate(),
            bpm: tag.bpm(),
            categories,
            category: tag.category().map(|c| c.to_owned()),
            channel_config: tag.channel_config(),
            comment: tag.comment().map(|c| c.to_owned()),
            comments,
            compilation: tag.compilation(),
            composer: tag.composer().map(|c| c.to_owned()),
            composers,
            copyright: tag.copyright().map(|c| c.to_owned()),
            custom_genre: tag.custom_genre().map(|c| c.to_owned()),
            custom_genres,
            description: tag.description().map(|d| d.to_owned()),
            descriptions,
            disc_number: tag.disc_number(),
            duration: tag.duration(),
            encoder: tag.encoder().map(|e| e.to_owned()),
            filetype: tag.filetype().to_owned(),
            gapless_playback: tag.gapless_playback(),
            genre: tag.genre().map(|g| g.to_owned()),
            genres,
            grouping: tag.grouping().map(|g| g.to_owned()),
            groupings,
            isrc: tag.isrc().map(|i| i.to_owned()),
            keyword: tag.keyword().map(|k| k.to_owned()),
            keywords,
            lyrics: tag.lyrics().map(|l| l.to_owned()),
            lyricist: tag.lyricist().map(|l| l.to_owned()),
            lyricists,
            max_bitrate: tag.max_bitrate(),
            media_type: tag.media_type(),
            movement: tag.movement().map(|m| m.to_owned()),
            movement_count: tag.movement_count(),
            movement_index: tag.movement_index(),
            sample_rate: tag.sample_rate(),
            standard_genre: tag.standard_genre(),
            standard_genres,
            title: tag.title().map(|t| t.to_owned()),
            total_discs: tag.total_discs(),
            total_tracks: tag.total_tracks(),
            track: tag.track(),
            track_number: tag.track_number(),
            tv_episode: tag.tv_episode(),
            tv_episode_name: tag.tv_episode_name().map(|t| t.to_owned()),
            tv_network_name: tag.tv_network_name().map(|t| t.to_owned()),
            tv_season: tag.tv_season(),
            tv_show_name: tag.tv_show_name().map(|t| t.to_owned()),
            tv_show_name_sort_order: tag.tv_show_name_sort_order().map(|t| t.to_owned()),
            work: tag.work().map(|w| w.to_owned()),
            year: tag.year().map(|y| y.to_owned()),
        })
    }

    pub fn print_full(&self) {
        println!(
            "advisory_rating = {}",
            option_advisory_rating(&self.advisory_rating)
        );
        println!("album = {}", option_string(&self.album));
        println!("album_artist = {}", option_string(&self.album_artist));
        println!("album_artists = {}", vec_string(&self.album_artists));
        println!("artist = {}", option_string(&self.artist));
        println!("artists = {}", vec_string(&self.artists));
        println!("avg_bitrate = {}", option_u32(&self.avg_bitrate));
        println!("bpm = {}", option_u16(&self.bpm));
        println!("categories = {}", vec_string(&self.categories));
        println!("category = {}", option_string(&self.category));
        println!(
            "channel_config = {}",
            option_channel_config(&self.channel_config)
        );
        println!("comment = {}", option_string(&self.comment));
        println!("comments = {}", vec_string(&self.comments));
        println!("compilation = {}", print_bool(self.compilation));
        println!("composer = {}", option_string(&self.composer));
        println!("composers = {}", vec_string(&self.composers));
        println!("copyright = {}", option_string(&self.copyright));
        println!("custom_genre = {}", option_string(&self.custom_genre));
        println!("custom_genres = {}", vec_string(&self.custom_genres));
        println!("description = {}", option_string(&self.description));
        println!("descriptions = {}", vec_string(&self.descriptions));
        println!("disc_number = {}", option_u16(&self.disc_number));
        println!(
            "duration = {}",
            pretty_duration::pretty_duration(&self.duration, None)
        );
        println!("encoder = {}", option_string(&self.encoder));
        println!("filetype = {}", self.filetype);
        println!("gapless_playback = {}", print_bool(self.gapless_playback));
        println!("genre = {}", option_string(&self.genre));
        println!("genres = {}", vec_string(&self.genres));
        println!("grouping = {}", option_string(&self.grouping));
        println!("groupings = {}", vec_string(&self.groupings));
        println!("isrc = {}", option_string(&self.isrc));
        println!("keyword = {}", option_string(&self.keyword));
        println!("keywords = {}", vec_string(&self.keywords));
        println!("lyrics = {}", option_string(&self.lyrics));
        println!("lyricist = {}", option_string(&self.lyricist));
        println!("lyricists = {}", vec_string(&self.lyricists));
        println!("max_bitrate = {}", option_u32(&self.max_bitrate));
        println!("media_type = {}", option_media_type(&self.media_type));
        println!("movement = {}", option_string(&self.movement));
        println!("movement_count = {}", option_u16(&self.movement_count));
        println!("movement_index = {}", option_u16(&self.movement_index));
        println!("sample_rate = {}", option_sample_rate(&self.sample_rate));
        println!("standard_genre = {}", option_u16(&self.standard_genre));
        println!("standard_genres = {}", vec_u16(&self.standard_genres));
        println!("title = {}", option_string(&self.title));
        println!("total_discs = {}", option_u16(&self.total_discs));
        println!("total_tracks = {}", option_u16(&self.total_tracks));
        println!(
            "track = {} ({})",
            option_u16(&self.track.0),
            option_u16(&self.track.1)
        );
        println!("track_number = {}", option_u16(&self.track_number));
        println!("tv_episode = {}", option_u32(&self.tv_episode));
        println!("tv_episode_name = {}", option_string(&self.tv_episode_name));
        println!("tv_network_name = {}", option_string(&self.tv_network_name));
        println!("tv_season = {}", option_u32(&self.tv_season));
        println!("tv_show_name = {}", option_string(&self.tv_show_name));
        println!(
            "tv_show_name_sort_order = {}",
            option_string(&self.tv_show_name_sort_order)
        );
        println!("work = {}", option_string(&self.work));
        println!("year = {}", option_string(&self.year));
    }

    /// Print information about what the file contains.
    pub fn print_what(&self) {
        println!("title = {}", option_string(&self.title));
        println!("album = {}", option_string(&self.album));
        println!("disc_number = {}", option_u16(&self.disc_number));
        println!(
            "track = {} ({})",
            option_u16(&self.track.0),
            option_u16(&self.track.1)
        );
        println!("custom_genres = {}", vec_string(&self.custom_genres));
        println!("genres = {}", vec_string(&self.genres));
        println!("standard_genres = {}", vec_u16(&self.standard_genres));
    }

    /// Print information about who created this.
    pub fn print_who(&self) {
        println!("artists = {}", vec_string(&self.artists));
        println!("album_artists = {}", vec_string(&self.album_artists));
        println!("composers = {}", vec_string(&self.composers));
        println!("copyright = {}", option_string(&self.copyright));
    }

    pub fn into_hashmap(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();

        map.insert(
            "advisory_rating",
            advisory_rating_with_default(&self.advisory_rating),
        );
        map.insert("album", string_with_default(&self.album, DEFAULT_ALBUM));
        map.insert(
            "album_artist",
            string_with_default(&self.album_artist, DEFAULT_ALBUM_ARTIST),
        );
        map.insert(
            "album_artists",
            vec_string_flat(&self.album_artists, DEFAULT_ALBUM_ARTIST),
        );
        map.insert("artist", string_with_default(&self.artist, DEFAULT_ARTIST));
        map.insert("artists", vec_string_flat(&self.artists, DEFAULT_ARTIST));
        map.insert(
            "avg_bitrate",
            u32_with_default(&self.avg_bitrate, DEFAULT_AVG_BITRATE),
        );
        map.insert("bpm", u16_with_default(&self.bpm, DEFAULT_BPM));
        map.insert(
            "categories",
            vec_string_flat(&self.categories, DEFAULT_CATEGORY),
        );
        map.insert(
            "category",
            string_with_default(&self.category, DEFAULT_CATEGORY),
        );
        map.insert(
            "channel_config",
            channel_config_with_default(&self.channel_config),
        );
        map.insert(
            "comment",
            string_with_default(&self.comment, DEFAULT_COMMENT),
        );
        map.insert("comments", vec_string_flat(&self.comments, DEFAULT_COMMENT));
        map.insert("compilation", print_bool(self.compilation));
        map.insert(
            "composer",
            string_with_default(&self.composer, DEFAULT_COMPOSER),
        );
        map.insert(
            "composers",
            vec_string_flat(&self.composers, DEFAULT_COMPOSER),
        );
        map.insert(
            "copyright",
            string_with_default(&self.copyright, DEFAULT_COPYRIGHT),
        );
        map.insert(
            "custom_genre",
            string_with_default(&self.custom_genre, DEFAULT_CUSTOM_GENRE),
        );
        map.insert(
            "custom_genres",
            vec_string_flat(&self.custom_genres, DEFAULT_CUSTOM_GENRE),
        );
        map.insert(
            "description",
            string_with_default(&self.description, DEFAULT_DESCRIPTION),
        );
        map.insert(
            "descriptions",
            vec_string_flat(&self.descriptions, DEFAULT_DESCRIPTION),
        );
        map.insert(
            "disc_number",
            u16_with_default(&self.disc_number, DEFAULT_DISC_NUMBER),
        );
        map.insert(
            "duration",
            pretty_duration::pretty_duration(&self.duration, None),
        );
        map.insert(
            "encoder",
            string_with_default(&self.encoder, DEFAULT_ENCODER),
        );
        map.insert("filetype", self.filetype.clone());
        map.insert("gapless_playback", print_bool(self.gapless_playback));
        map.insert("genre", string_with_default(&self.genre, DEFAULT_GENRE));
        map.insert("genres", vec_string_flat(&self.genres, DEFAULT_GENRE));
        map.insert(
            "grouping",
            string_with_default(&self.grouping, DEFAULT_GROUPING),
        );
        map.insert(
            "groupings",
            vec_string_flat(&self.groupings, DEFAULT_GROUPING),
        );
        map.insert("isrc", string_with_default(&self.isrc, DEFAULT_ISRC));
        map.insert(
            "keyword",
            string_with_default(&self.keyword, DEFAULT_KEYWORD),
        );
        map.insert("keywords", vec_string_flat(&self.keywords, DEFAULT_KEYWORD));
        map.insert("lyrics", string_with_default(&self.lyrics, DEFAULT_LYRICS));
        map.insert(
            "lyricist",
            string_with_default(&self.lyricist, DEFAULT_LYRICIST),
        );
        map.insert(
            "lyricists",
            vec_string_flat(&self.lyricists, DEFAULT_LYRICIST),
        );
        map.insert(
            "max_bitrate",
            u32_with_default(&self.max_bitrate, DEFAULT_MAX_BITRATE),
        );
        map.insert("media_type", media_type_with_default(&self.media_type));
        map.insert(
            "movement",
            string_with_default(&self.movement, DEFAULT_MOVEMENT),
        );
        map.insert(
            "movement_count",
            u16_with_default(&self.movement_count, DEFAULT_MOVEMENT_COUNT),
        );
        map.insert(
            "movement_index",
            u16_with_default(&self.movement_index, DEFAULT_MOVEMENT_INDEX),
        );
        map.insert("sample_rate", sample_rate_with_default(&self.sample_rate));
        map.insert(
            "standard_genre",
            u16_with_default(&self.standard_genre, DEFAULT_STANDARD_GENRE),
        );
        map.insert(
            "standard_genres",
            vec_u16_flat(&self.standard_genres, DEFAULT_STANDARD_GENRE),
        );
        map.insert("title", string_with_default(&self.title, DEFAULT_TITLE));
        map.insert(
            "total_discs",
            u16_with_default(&self.total_discs, DEFAULT_TOTAL_DISCS),
        );
        map.insert(
            "total_tracks",
            u16_with_default(&self.total_tracks, DEFAULT_TOTAL_TRACKS),
        );
        map.insert(
            "track",
            format!(
                "{} ({})",
                u16_with_default(&self.track.0, DEFAULT_TRACK_NUMBER),
                u16_with_default(&self.track.1, DEFAULT_TOTAL_TRACKS)
            ),
        );
        map.insert(
            "track_number",
            u16_with_default(&self.track_number, DEFAULT_TRACK_NUMBER),
        );
        map.insert(
            "tv_episode",
            u32_with_default(&self.tv_episode, DEFAULT_TV_EPISODE),
        );
        map.insert(
            "tv_episode_name",
            string_with_default(&self.tv_episode_name, DEFAULT_TV_EPISODE_NAME),
        );
        map.insert(
            "tv_network_name",
            string_with_default(&self.tv_network_name, DEFAULT_TV_NETWORK_NAME),
        );
        map.insert(
            "tv_season",
            u32_with_default(&self.tv_season, DEFAULT_TV_SEASON),
        );
        map.insert(
            "tv_show_name",
            string_with_default(&self.tv_show_name, DEFAULT_TV_SHOW_NAME),
        );
        map.insert(
            "tv_show_name_sort_order",
            string_with_default(
                &self.tv_show_name_sort_order,
                DEFAULT_TV_SHOW_NAME_SORT_ORDER,
            ),
        );
        map.insert("work", string_with_default(&self.work, DEFAULT_WORK));
        map.insert("year", string_with_default(&self.year, DEFAULT_YEAR));

        map
    }

    pub fn defaults_hashmap<'a>() -> HashMap<&'a str, String> {
        let mut map = HashMap::new();

        map.insert("advisory_rating", DEFAULT_ADVISORY_RATING.to_string());
        map.insert("album", DEFAULT_ALBUM.to_string());
        map.insert("album_artist", DEFAULT_ALBUM_ARTIST.to_string());
        map.insert("album_artists", DEFAULT_ALBUM_ARTIST.to_string());
        map.insert("artist", DEFAULT_ARTIST.to_string());
        map.insert("artists", DEFAULT_ARTIST.to_string());
        map.insert("avg_bitrate", DEFAULT_AVG_BITRATE.to_string());
        map.insert("bpm", DEFAULT_BPM.to_string());
        map.insert("categories", DEFAULT_CATEGORY.to_string());
        map.insert("category", DEFAULT_CATEGORY.to_string());
        map.insert("channel_config", DEFAULT_CHANNEL_CONFIG.to_string());
        map.insert("comment", DEFAULT_COMMENT.to_string());
        map.insert("comments", DEFAULT_COMMENT.to_string());
        map.insert("composer", DEFAULT_COMPOSER.to_string());
        map.insert("composers", DEFAULT_COMPOSER.to_string());
        map.insert("copyright", DEFAULT_COPYRIGHT.to_string());
        map.insert("custom_genre", DEFAULT_CUSTOM_GENRE.to_string());
        map.insert("custom_genres", DEFAULT_CUSTOM_GENRE.to_string());
        map.insert("description", DEFAULT_DESCRIPTION.to_string());
        map.insert("descriptions", DEFAULT_DESCRIPTION.to_string());
        map.insert("disc_number", DEFAULT_DISC_NUMBER.to_string());
        map.insert("duration", String::from("0s"));
        map.insert("encoder", DEFAULT_ENCODER.to_string());
        map.insert("genre", DEFAULT_GENRE.to_string());
        map.insert("genres", DEFAULT_GENRE.to_string());
        map.insert("grouping", DEFAULT_GROUPING.to_string());
        map.insert("groupings", DEFAULT_GROUPING.to_string());
        map.insert("isrc", DEFAULT_ISRC.to_string());
        map.insert("keyword", DEFAULT_KEYWORD.to_string());
        map.insert("keywords", DEFAULT_KEYWORD.to_string());
        map.insert("lyrics", DEFAULT_LYRICS.to_string());
        map.insert("lyricist", DEFAULT_LYRICIST.to_string());
        map.insert("lyricists", DEFAULT_LYRICIST.to_string());
        map.insert("max_bitrate", DEFAULT_MAX_BITRATE.to_string());
        map.insert("media_type", DEFAULT_MEDIA_TYPE.to_string());
        map.insert("movement", DEFAULT_MOVEMENT.to_string());
        map.insert("movement_count", DEFAULT_MOVEMENT_COUNT.to_string());
        map.insert("movement_index", DEFAULT_MOVEMENT_INDEX.to_string());
        map.insert("sample_rate", DEFAULT_SAMPLE_RATE.to_string());
        map.insert("standard_genre", DEFAULT_GENRE.to_string());
        map.insert("standard_genres", DEFAULT_GENRE.to_string());
        map.insert("title", DEFAULT_TITLE.to_string());
        map.insert("total_discs", DEFAULT_TOTAL_DISCS.to_string());
        map.insert("total_tracks", DEFAULT_TOTAL_TRACKS.to_string());
        map.insert(
            "track",
            format!("{} ({})", DEFAULT_TRACK_NUMBER, DEFAULT_TOTAL_TRACKS),
        );
        map.insert("track_number", DEFAULT_TRACK_NUMBER.to_string());
        map.insert("tv_episode", DEFAULT_TV_EPISODE.to_string());
        map.insert("tv_episode_name", DEFAULT_TV_EPISODE_NAME.to_string());
        map.insert("tv_network_name", DEFAULT_TV_NETWORK_NAME.to_string());
        map.insert("tv_season", DEFAULT_TV_SEASON.to_string());
        map.insert("tv_show_name", DEFAULT_TV_SHOW_NAME.to_string());
        map.insert(
            "tv_show_name_sort_order",
            DEFAULT_TV_SHOW_NAME_SORT_ORDER.to_string(),
        );
        map.insert("work", DEFAULT_WORK.to_string());
        map.insert("year", DEFAULT_YEAR.to_string());

        map
    }
}

fn print_bool(input: bool) -> String {
    if input {
        String::from("yes")
    } else {
        String::from("no")
    }
}

fn option_string(input: &Option<String>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::from(""),
    }
}

fn option_u16(input: &Option<u16>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::from(""),
    }
}

fn option_u32(input: &Option<u32>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::from(""),
    }
}

fn vec_string(input: &Vec<String>) -> String {
    if input.len() == 0 {
        return String::from("(0)");
    }

    let mut output = String::new();

    if input.len() == 1 {
        output.push_str("(1) ");
        output.push_str(input.first().expect("has length"));
        return output;
    }

    output.push('(');
    output.push_str(input.len().to_string().as_str());
    output.push(')');

    for item in input {
        output.push_str("\n    ");
        output.push_str(item);
    }

    output
}

fn vec_u16(input: &Vec<u16>) -> String {
    if input.len() == 0 {
        return String::from("(0)");
    }

    let mut output = String::new();

    if input.len() == 1 {
        output.push_str("(1) ");
        output.push_str(&format!("{}", input.first().expect("has length")));
        return output;
    }

    output.push('(');
    output.push_str(input.len().to_string().as_str());
    output.push(')');

    for item in input {
        output.push_str("\n    ");
        output.push_str(item.to_string().as_str());
    }

    output
}

fn option_advisory_rating(input: &Option<AdvisoryRating>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::new(),
    }
}

fn option_media_type(input: &Option<MediaType>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::new(),
    }
}

fn option_channel_config(input: &Option<ChannelConfig>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::new(),
    }
}

fn option_sample_rate(input: &Option<SampleRate>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::new(),
    }
}

fn advisory_rating_with_default(input: &Option<AdvisoryRating>) -> String {
    match input {
        Some(a) => format!("{}", a),
        None => String::from(DEFAULT_ADVISORY_RATING),
    }
}

fn channel_config_with_default(input: &Option<ChannelConfig>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::from(DEFAULT_CHANNEL_CONFIG),
    }
}

fn media_type_with_default(input: &Option<MediaType>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::from(DEFAULT_MEDIA_TYPE),
    }
}

fn sample_rate_with_default(input: &Option<SampleRate>) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => String::from(DEFAULT_SAMPLE_RATE),
    }
}

fn string_with_default(input: &Option<String>, default: &str) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => default.to_owned(),
    }
}

fn u16_with_default(input: &Option<u16>, default: &str) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => default.to_owned(),
    }
}

fn u32_with_default(input: &Option<u32>, default: &str) -> String {
    match input {
        Some(i) => format!("{}", i),
        None => default.to_owned(),
    }
}

fn vec_string_flat(input: &Vec<String>, default: &str) -> String {
    if input.len() == 0 {
        return default.to_owned();
    }

    input.join(", ")
}

fn vec_u16_flat(input: &Vec<u16>, default: &str) -> String {
    if input.len() == 0 {
        return default.to_owned();
    }

    input
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
