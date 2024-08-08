use once_cell::sync::Lazy;
use std::{collections::HashMap, str::FromStr};

use codewars::*;

pub static MORSE_CODE: Lazy<HashMap<String, String>> = Lazy::new(
    || map_ss!("--.-": "Q", "-.-.--": "!", "---..": "8", "-....": "6", "....-": "4", "-....-": "-", ".--": "W", "-.--.-": ")", "...-": "V", ".----.": "'", "..": "I", "...": "S", ".-.": "R", "--..": "Z", ".....": "5", "----.": "9", "--..--": ",", "-..-.": "/", "-.--.": "(", "---...": ",", "-.-.-.": ";", "-": "T", ".-.-.-": ".", "-..-": "X", ".-.-.": "+", "..--..": "?", "..--.-": "_", "...---...": "SOS", ".-..": "L", "-...": "B", "..-": "U", "--.": "G", "-.": "N", "-.--": "Y", ".": "E", "--...": "7", "-..": "D", ".----": "1", ".-": "A", ".-...": "&", "...-..-": "$", ".--.-.": "@", "-...-": "=", "....": "H", ".---": "J", "---": "O", ".--.": "P", "-----": "0", ".-..-.": "\"", "-.-.": "C", "..---": "2", "-.-": "K", "--": "M", "..-.": "F", "...--": "3"),
);
