use walkdir::WalkDir;
use std::path::Path;
use id3::{Tag, TagLike};
use regex;

#[derive(Debug)]
#[allow(dead_code)]
struct MediaFile {
    artist: String,
    album: String,
    song: String,
    track: String,
    disk: String,
    ext: String,
    dir: String,
    file: String,
    
}

fn main() {
    let dir_path = "/media/pinas/foo1/Music/Music/F".to_string();
    // let dir_path = "/home/pipi/Music/".to_string();

    let media_files = find_media(&dir_path);

    for afile in media_files {
        let (dir, file) = split_path(&afile).unwrap();
        let (artist, album, song) = get_filename_artist_album_song(file.clone());
        let ext = get_extension(&file);
        let filename_info = MediaFile {
            artist: artist,
            album: album,
            song: song,
            track: get_filename_track(file.clone()),
            disk: get_filename_disk(file.clone()),
            ext: ext.clone(),
            dir: dir.clone(),
            file: file.clone(),
        };
        // println!("filename_info:{:#?}", filename_info);

        let tag_info = MediaFile{
            artist: get_tag_artist(afile.clone()),
            album: get_tag_album(afile.clone()),
            song: get_tag_title(afile.clone()),
            track: get_tag_track(afile.clone()),
            disk: get_tag_TPOS(afile.clone()),
            ext,
            dir,
            file,
        };
        // println!("{:#?}", tag_info);

        compare_media_files(&filename_info, &tag_info);
    }
}

fn compare_media_files(file1: &MediaFile, file2: &MediaFile) {
    if file1.artist != file2.artist {
        println!("Artist differs: {} != {}", file1.artist, file2.artist);
    }
    if file1.album != file2.album {
        println!("Album differs: {} != {}", file1.album, file2.album);
    }
    if file1.song != file2.song {
        println!("Song differs: {} != {}", file1.song, file2.song);
        println!("{}", file1.file);
    }
    if file1.track != file2.track {
        println!("Track differs: {} != {}", file1.track, file2.track);
        println!("{}", file1.file);
    }
    // if file1.disk != file2.disk {
    //     println!("Disk differs: {} != {}", file1.disk, file2.disk);
    // }
    if file1.ext != file2.ext {
        println!("Extension differs: {} != {}", file1.ext, file2.ext);
    }
    if file1.dir != file2.dir {
        println!("Directory differs: {} != {}", file1.dir, file2.dir);
    }
    if file1.file != file2.file {
        println!("File differs: {} != {}", file1.file, file2.file);
    }
}

#[allow(non_snake_case)]
fn get_tag_TPOS(apath: String) -> String {
    let tag = Tag::read_from_path(apath).unwrap();
    let tpos = tag.get("TPOS")
        .and_then(|frame| frame.content().text())
        .unwrap_or("")
        .to_string();
    let tpos_cleaned = clean_filename(tpos);

    tpos_cleaned
}

fn get_tag_track(apath: String) -> String {
    let tag = Tag::read_from_path(apath).unwrap();
    let track = tag.track().unwrap_or(0).to_string();
    let track_cleaned = clean_filename(track);
    if track_cleaned == "1".to_owned() {
        return "01".to_string()
    } else if track_cleaned == "2".to_owned() {
        return "02".to_string();
    } else if track_cleaned == "3".to_owned() {
        return "03".to_string();
    } else if track_cleaned == "4".to_owned() {
        return "04".to_string();
    } else if track_cleaned == "5".to_owned() {
        return "05".to_string();
    } else if track_cleaned == "6".to_owned() {
        return "06".to_string();
    } else if track_cleaned == "7".to_owned() {
        return "07".to_string();
    } else if track_cleaned == "8".to_owned() {
        return "08".to_string();
    } else if track_cleaned == "9".to_owned() {
        return "09".to_string();
    } else {
        return track_cleaned;
    }
}

fn get_tag_title(apath: String) -> String {
    let tag = Tag::read_from_path(apath).unwrap();
    let song = tag.title().unwrap_or("").to_string();
    let song_cleaned = clean_filename(song);

    song_cleaned
}

fn get_tag_album(apath: String) -> String {
    let tag = Tag::read_from_path(apath).unwrap();
    let album = tag.album().unwrap_or("").to_string();
    let album_cleaned = clean_filename(album);

    album_cleaned
}

fn get_tag_artist(apath: String) -> String {
    let tag = Tag::read_from_path(apath).unwrap();
    let artist = tag.artist().unwrap_or("").to_string();
    let artist_cleaned = clean_filename(artist);

    artist_cleaned
}

fn get_filename_artist_album_song(apath: String) -> (String, String, String) {
    let parts: Vec<&str> = apath.split("_-_").collect();
    let artist = parts.get(1).unwrap_or(&"").to_string();
    let album = parts.get(2).unwrap_or(&"").to_string();
    let song = parts.get(3).unwrap_or(&"").to_string();
    let artist_cleaned = clean_filename(artist);
    let album_cleaned = clean_filename(album);
    let mut song_cleaned = clean_filename(song);
    song_cleaned = song_cleaned.replace(".mp3", "");

    (
        artist_cleaned.replace("_", " "), 
        album_cleaned.replace("_", " "),
        song_cleaned.replace("_", " "),
    )
}

fn get_extension(apath: &String) -> String {
    let path = Path::new(apath);
    path.extension()
        .map(|ext| ext.to_str().unwrap_or(""))
        .unwrap_or("")
        .to_string()
}

fn get_filename_track(apath: String) -> String {
    let mut chars = apath.chars();
    let second_char = chars.nth(2).unwrap_or_default();
    let third_char = chars.nth(0).unwrap_or_default();
    format!("{}{}", second_char, third_char)
}

fn get_filename_disk(apath: String) -> String {
    let disk_num = apath.chars().next().unwrap().to_string();
    if disk_num == "1".to_owned() {
        return "01".to_string()
    } else if disk_num == "2".to_owned() {
        return "02".to_string();
    } else if disk_num == "3".to_owned() {
        return "03".to_string();
    } else if disk_num == "4".to_owned() {
        return "04".to_string();
    } else if disk_num == "5".to_owned() {
        return "05".to_string();
    } else if disk_num == "6".to_owned() {
        return "06".to_string();
    } else if disk_num == "7".to_owned() {
        return "07".to_string();
    } else if disk_num == "8".to_owned() {
        return "08".to_string();
    } else if disk_num == "9".to_owned() {
        return "09".to_string();
    } else {
        return disk_num;
    }
}

fn clean_filename(apath: String) -> String {
    // let file_parts = split_path(&apath).unwrap();
    // let file_name1 = file_parts.1;
    let file_name2 = remove_parentheses_and_contents(&apath);
    let file_name = file_name2.replace("&", "And")
        .replace("+", "And")
        .replace(" .mp3", ".mp3")
        .replace("'", "")
        .replace(",", "")
        .replace("?", "")
        .replace(". ", "_-_");
        
    // let file_name4 = file_parts.0 + "/" + &file_name3;
    // let file_name = file_name4.replace(" ", "_");

    // let _ = rename_file(apath, file_name.clone());
    // println!("File name: {:?}", file_name);



    file_name
}

fn remove_parentheses_and_contents(input: &str) -> String {
    // Define the regular expression to match parentheses and their contents
    let re = regex::Regex::new(r"\([^)]*\)").unwrap(); 

    // Replace all matches with an empty string
    re.replace_all(input, "").to_string()
}


fn split_path(path_str: &str) -> Option<(String, String)> {
    let path = Path::new(path_str);

    let dir = path
        .parent()
        .map(|p| p.to_str().unwrap_or(""))
        .unwrap_or("."); // Handle root paths

    let file = path
        .file_name()
        .map(|f| f.to_str().unwrap_or(""))
        .unwrap_or("");

    Some((dir.to_string(), file.to_string()))
}

pub fn find_media(dir_path: &String) -> Vec<String> {
    println!("Dir path: {:?}", dir_path);
    let mut media_files = Vec::new();
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| {
            ext == "mp3"
                || ext == "MP3"
                || ext == "flac"
                || ext == "FLAC"
                || ext == "ogg"
                || ext == "OGG"
                || ext == "wav"
                || ext == "WAV"
        }) {
            media_files.push(entry.path().to_string_lossy().into_owned());
        }
    }

    media_files
}