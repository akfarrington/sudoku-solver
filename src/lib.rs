use std::fmt;
use std::collections::HashMap;
use serde::Deserialize;

const MEDIUM_BOX_1: [u8; 9] = [0, 1, 2, 9, 10, 11, 18, 19, 20];
const MEDIUM_BOX_2: [u8; 9] = [3, 4, 5, 12, 13, 14, 21, 22, 23];
const MEDIUM_BOX_3: [u8; 9] = [6, 7, 8, 15, 16, 17, 24, 25, 26];

const MEDIUM_BOX_4: [u8; 9] = [27, 28, 29, 36, 37, 38, 45, 46, 47];
const MEDIUM_BOX_5: [u8; 9] = [30, 31, 32, 39, 40, 41, 48, 49, 50];
const MEDIUM_BOX_6: [u8; 9] = [33, 34, 35, 42, 43, 44, 51, 52, 53];

const MEDIUM_BOX_7: [u8; 9] = [54, 55, 56, 63, 64, 65, 72, 73, 74];
const MEDIUM_BOX_8: [u8; 9] = [57, 58, 59, 66, 67, 68, 75, 76, 77];
const MEDIUM_BOX_9: [u8; 9] = [60, 61, 62, 69, 70, 71, 78, 79, 80];


const ROW_STARTERS: [u8; 9] = [0, 9, 18, 27, 36, 45, 54, 63, 72];
const COL_STARTERS: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const BOX_STARTERS: [u8; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];

const ALL_NOTES_POSSIBILITIES: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

#[derive(Debug)]
pub struct LittleBox {
    value: u8,
    original_value: u8,
    index: u8,
    one_p: bool,
    two_p: bool,
    three_p: bool,
    four_p: bool,
    five_p: bool,
    six_p: bool,
    seven_p: bool,
    eight_p: bool,
    nine_p: bool,
}

pub struct BigBox {
    pub little_boxes: Vec<LittleBox>,
    pub times_updated: u32,
}

#[derive(Deserialize)]
pub struct WebForm {
    pub cell_0: String,
    pub cell_1: String,
    pub cell_2: String,
    pub cell_3: String,
    pub cell_4: String,
    pub cell_5: String,
    pub cell_6: String,
    pub cell_7: String,
    pub cell_8: String,
    pub cell_9: String,
    pub cell_10: String,
    pub cell_11: String,
    pub cell_12: String,
    pub cell_13: String,
    pub cell_14: String,
    pub cell_15: String,
    pub cell_16: String,
    pub cell_17: String,
    pub cell_18: String,
    pub cell_19: String,
    pub cell_20: String,
    pub cell_21: String,
    pub cell_22: String,
    pub cell_23: String,
    pub cell_24: String,
    pub cell_25: String,
    pub cell_26: String,
    pub cell_27: String,
    pub cell_28: String,
    pub cell_29: String,
    pub cell_30: String,
    pub cell_31: String,
    pub cell_32: String,
    pub cell_33: String,
    pub cell_34: String,
    pub cell_35: String,
    pub cell_36: String,
    pub cell_37: String,
    pub cell_38: String,
    pub cell_39: String,
    pub cell_40: String,
    pub cell_41: String,
    pub cell_42: String,
    pub cell_43: String,
    pub cell_44: String,
    pub cell_45: String,
    pub cell_46: String,
    pub cell_47: String,
    pub cell_48: String,
    pub cell_49: String,
    pub cell_50: String,
    pub cell_51: String,
    pub cell_52: String,
    pub cell_53: String,
    pub cell_54: String,
    pub cell_55: String,
    pub cell_56: String,
    pub cell_57: String,
    pub cell_58: String,
    pub cell_59: String,
    pub cell_60: String,
    pub cell_61: String,
    pub cell_62: String,
    pub cell_63: String,
    pub cell_64: String,
    pub cell_65: String,
    pub cell_66: String,
    pub cell_67: String,
    pub cell_68: String,
    pub cell_69: String,
    pub cell_70: String,
    pub cell_71: String,
    pub cell_72: String,
    pub cell_73: String,
    pub cell_74: String,
    pub cell_75: String,
    pub cell_76: String,
    pub cell_77: String,
    pub cell_78: String,
    pub cell_79: String,
    pub cell_80: String
}

impl LittleBox {
    pub fn new(value: u8, index: u8) -> Option<LittleBox> {
        if value > 9 {
            return None;
        }
        if value == 0 {
            return Some(LittleBox{
                value: value,
                original_value: value,
                index: index,
                one_p: true,
                two_p: true,
                three_p: true,
                four_p: true,
                five_p: true,
                six_p: true,
                seven_p: true,
                eight_p: true,
                nine_p: true,
            })
        } else {
            return Some(LittleBox{
                value: value,
                original_value: value,
                index: index,
                one_p: false,
                two_p: false,
                three_p: false,
                four_p: false,
                five_p: false,
                six_p: false,
                seven_p: false,
                eight_p: false,
                nine_p: false,
            })
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_pretty_value(&self) -> String {
        let value = self.get_value();
        if value == 0 {
            format!(" ")
        } else {
            format!("{}", self.get_value())
        }
    }

    pub fn update_value(&mut self, value: u8) {
        self.value = value;
        self.make_all_impossible();
    }

    pub fn make_impossible(&mut self, number: u8) -> bool {
        let possibles = self.get_possibles();
        if possibles.contains(&number) {
            match number{
                1 => self.one_p = false,
                2 => self.two_p = false,
                3 => self.three_p = false,
                4 => self.four_p = false,
                5 => self.five_p = false,
                6 => self.six_p = false,
                7 => self.seven_p = false,
                8 => self.eight_p = false,
                9 => self.nine_p = false,
                _ => self.value = 42, // this could never happen
            }
            true
        } else {
            false
        }
    }

    pub fn make_many_impossible(&mut self, list: Vec<u8>) -> bool {
        let mut was_updated = false;
        for i in list {
            if self.make_impossible(i) {
                was_updated = true;
            }
        }

        was_updated
    }

    pub fn make_all_impossible(&mut self){
        for i in 1..=9{
            self.make_impossible(i);
        }
    }

    // this is if something bad happens and the board needs to be rerun
    pub fn make_all_possible(&mut self){
        self.one_p = true;
        self.two_p = true;
        self.three_p = true;
        self.four_p = true;
        self.five_p = true;
        self.six_p = true;
        self.seven_p = true;
        self.eight_p = true;
        self.nine_p = true;
    }

    pub fn get_possibles(&self) -> Vec<u8> {
        let mut possibles: Vec<u8> = Vec::new();

        if self.one_p == true {
            possibles.push(1);
        }
        if self.two_p == true {
            possibles.push(2);
        }
        if self.three_p == true {
            possibles.push(3);
        }
        if self.four_p == true {
            possibles.push(4);
        }
        if self.five_p == true {
            possibles.push(5);
        }
        if self.six_p == true {
            possibles.push(6);
        }
        if self.seven_p == true {
            possibles.push(7);
        }
        if self.eight_p == true {
            possibles.push(8);
        }
        if self.nine_p == true {
            possibles.push(9);
        }

        possibles
    }

    pub fn pretty_print(&self) -> String {
        if self.get_value() == 0 {
            let this_notes = self.get_possibles();
            let mut return_string: String = String::new();

            for value in 0..=9 {
                match value {
                    1 => {
                        if this_notes.contains(&1) {
                            return_string += &"1".to_string();
                        } else {
                            return_string += &"&nbsp;".to_string();
                        }
                    },
                    2 => {
                        if this_notes.contains(&2) {
                            return_string += &"2".to_string();
                        } else {
                            return_string += &"&nbsp;".to_string();
                        }
                    },
                    3 => {
                        if this_notes.contains(&3) {
                            return_string += &"3<br />".to_string();
                        } else {
                            return_string += &"&nbsp;<br />".to_string();
                        }
                    },
                    4 => {
                        if this_notes.contains(&4) {
                            return_string += &"4".to_string();
                        } else {
                            return_string += &"&nbsp;".to_string();
                        }
                    },
                    5 => {
                        if this_notes.contains(&5) {
                            return_string += &"5".to_string();
                        } else {
                            return_string += &"&nbsp;".to_string();
                        }
                    },
                    6 => {
                        if this_notes.contains(&6) {
                            return_string += &"6<br />".to_string();
                        } else {
                            return_string += &"&nbsp;<br />".to_string();
                        }
                    },
                    7 => {
                        if this_notes.contains(&7) {
                            return_string += &"7".to_string();
                        } else {
                            return_string += &"&nbsp;".to_string();
                        }
                    },
                    8 => {
                        if this_notes.contains(&8) {
                            return_string += &"8".to_string();
                        } else {
                            return_string += &"&nbsp;".to_string();
                        }
                    },
                    9 => {
                        if this_notes.contains(&9) {
                            return_string += &"9".to_string();
                        } else {
                            return_string += &"&nbsp;".to_string();
                        }
                    },
                    _ => {}
                }                
            }
            return format!("<div class=\"notes\">{}</div>", return_string);
        } else {
            return format!("<input maxlength=\"1\" name=\"name_{}\" type=\"number\" value=\"{}\" disabled>",
                self.index, self.get_value());
        }
    }
}

impl fmt::Display for LittleBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BigBox {
    pub fn new(values: Vec<u8>) -> Option<BigBox> {
        if values.len() != 81 {
            println!("Puzzle length is incorrect. It's {} boxes long.", values.len());
            return None;
        }

        let mut little_box_values: Vec<LittleBox> = Vec::new();

        for (index, value) in values.iter().enumerate() {
            let new_little_box = LittleBox::new(*value, index as u8);
            match new_little_box{
                Some(lb) => {
                    little_box_values.push(lb);
                }
                None => {
                    return None;
                }
            }
            
        }

        Some(BigBox{
            little_boxes: little_box_values,
            times_updated: 0,
        })
    }

    // set a little box value but check if the insert is illegal, if so, force a rechecking
    // of possibilities
    pub fn update_little_box_value(&mut self, index: &u8, value: &u8) {
        let mut illegal = false;
        for index in BigBox::get_all_affected_indices(*index) {
            if &self.little_boxes[index as usize].value == value {
                illegal = true;
            }
        }

        if illegal {
            self.make_all_cells_all_possibilities_possible();
            self.mark_impossible(self.scan_for_make_impossible());
        } else {
            self.little_boxes[*index as usize].update_value(*value);
            self.set_times_updated_plus_one();
        }
    }

    // get times_updated
    pub fn get_times_updated(&self) -> u32{
        self.times_updated
    }

    // set times updated to +1
    pub fn set_times_updated_plus_one(&mut self){
        self.times_updated = self.times_updated + 1;
    }

    // this gets box indexes given an index
    pub fn get_box_indices(index: &u8) -> Vec<u8> {
        let mut affected_indices: Vec<u8> = Vec::new();
        if MEDIUM_BOX_1.contains(&index){
            for i in &MEDIUM_BOX_1 {
                affected_indices.push(*i);
            }
        } else if MEDIUM_BOX_2.contains(&index){
            for i in &MEDIUM_BOX_2 {
                affected_indices.push(*i);
            }
        } else if MEDIUM_BOX_3.contains(&index){
            for i in &MEDIUM_BOX_3 {
                affected_indices.push(*i);
            }
        } else if MEDIUM_BOX_4.contains(&index){
            for i in &MEDIUM_BOX_4 {
                affected_indices.push(*i);
            }
        } else if MEDIUM_BOX_5.contains(&index){
            for i in &MEDIUM_BOX_5 {
                affected_indices.push(*i);
            }
        } else if MEDIUM_BOX_6.contains(&index){
            for i in &MEDIUM_BOX_6 {
                affected_indices.push(*i);
            }
        } else if MEDIUM_BOX_7.contains(&index){
            for i in &MEDIUM_BOX_7 {
                affected_indices.push(*i);
            }
        } else if MEDIUM_BOX_8.contains(&index){
            for i in &MEDIUM_BOX_8 {
                affected_indices.push(*i);
            }
        } else if MEDIUM_BOX_9.contains(&index){
            for i in &MEDIUM_BOX_9 {
                affected_indices.push(*i);
            }
        }

        affected_indices
    }

    pub fn get_row_indices(index: &u8) -> Vec<u8> {
        let row_number = index / 9;
        let start = row_number * 9;
        (start..=start+8).collect::<Vec<u8>>()
    }

    pub fn get_col_indices(index: &u8) -> Vec<u8> {
        let y = index / 9;
        let x = index - (9 * y);

        (0..=8).map(|i| (i * 9) + x).collect::<Vec<u8>>()
    }

    pub fn get_all_affected_indices(index: u8) -> Vec<u8> {
        let mut affected_indices: Vec<u8> = Vec::new();

        for i in BigBox::get_box_indices(&index){
            affected_indices.push(i);
        }
        for i in BigBox::get_col_indices(&index){
            if !affected_indices.contains(&i){
                affected_indices.push(i)
            }
        }
        for i in BigBox::get_row_indices(&index){
            if !affected_indices.contains(&i){
                affected_indices.push(i);
            }
        }

        affected_indices.sort();
        affected_indices
    }

    pub fn make_all_cells_all_possibilities_possible(&mut self){
        for index in 0..=80 {
            if self.little_boxes[index as usize].get_value() == 0 {
                self.little_boxes[index as usize].make_all_possible();
            }
        }
    }

    // this is a first run thing, and shouldn't have to be run many times
    pub fn scan_for_make_impossible(&self) -> Vec<(u8, u8)>{
        // format of tuple is index, value
        let mut affected_list: Vec<(u8, u8)> = Vec::new();
        for (i, value) in self.little_boxes.iter().enumerate() {
            if value.value != 0 {
                for affected in BigBox::get_all_affected_indices(i as u8){
                    // self.little_boxes[affected as usize].make_impossible(value.value);
                    affected_list.push((affected, value.value));
                }
            }
        }
        affected_list
    }

    // this is a first run thing, and shouldn't have to be run many times
    pub fn mark_impossible(&mut self, impossible_list: Vec<(u8, u8)>){
        for (index, number) in impossible_list {
            self.little_boxes[index as usize].make_impossible(number);
        }
    }

    // scan each item in the puzzle to see what has only one possible solution
    pub fn scan_one_possible(&self) -> Vec<(u8, u8)>{
        let mut one_possible_list: Vec<(u8, u8)> = Vec::new();
        for index in 0..=80 {
            if self.little_boxes[index].value == 0 {
                let possibles = self.little_boxes[index].get_possibles();

                // count number of possibilities and if 1, add it to the list
                if possibles.len() == 1 {
                    one_possible_list.push((index as u8, possibles[0]))
                }
            }
        }

        one_possible_list
    }

    // scan for only one box can have a value in a row/col or med box
    pub fn scan_for_one_possible_in_group(&mut self, index: u8){
        // check for box
        let this_possible_list = self.little_boxes[index as usize].get_possibles();
        let mut box_other_possible_list: Vec<u8> = Vec::new();

        for i in BigBox::get_box_indices(&index){
            if i != index {
                box_other_possible_list.append(&mut self.little_boxes[i as usize].get_possibles());
            }
        }
        for this_possible in &this_possible_list {
            if !box_other_possible_list.contains(&this_possible){
                self.add_valid_values(vec!((index, *this_possible)));
            }
        }

        // go again for columns
        let mut box_other_possible_list: Vec<u8> = Vec::new();

        for i in BigBox::get_col_indices(&index){
            if i != index {
                box_other_possible_list.append(& mut self.little_boxes[i as usize].get_possibles());
            }
        }
        for this_possible in &this_possible_list {
            if !box_other_possible_list.contains(&this_possible){
                self.add_valid_values(vec!((index, *this_possible)));
            }
        }

        // go again for rows
        let mut box_other_possible_list: Vec<u8> = Vec::new();
        for i in BigBox::get_row_indices(&index){
            if i != index {
                box_other_possible_list.append(&mut self.little_boxes[i as usize].get_possibles());
            }
        }
        for this_possible in &this_possible_list {
            if !box_other_possible_list.contains(&this_possible){
                self.add_valid_values(vec!((index, *this_possible)));
            }
        }
    }

    // check if a medium box has a row or column with unique values to remove
    // possible candidates from that same row or column in other medium boxes
    pub fn find_medium_box_row_col_unique_possibles(&mut self, med_box: [u8; 9]){
        /*
        The logic for this one doesn't have to be hard. Do it like this:
        match the medium box by index
        0 => get row1 and column1,
        1 => get column2,
        2 => get column3,
        4 => get row2,
        7 => get row3,
        */

        // make row and column index lists
        let row1: Vec<u8> = vec!(med_box[0], med_box[1], med_box[2]);
        let row2: Vec<u8> = vec!(med_box[3], med_box[4], med_box[5]);
        let row3: Vec<u8> = vec!(med_box[6], med_box[7], med_box[8]);

        let col1: Vec<u8> = vec!(med_box[0], med_box[3], med_box[6]);
        let col2: Vec<u8> = vec!(med_box[1], med_box[4], med_box[7]);
        let col3: Vec<u8> = vec!(med_box[2], med_box[5], med_box[8]);

        // get row and column possibilities lists
        let row1_p = self.get_group_possibles(&row1);
        let row2_p = self.get_group_possibles(&row2);
        let row3_p = self.get_group_possibles(&row3);

        let col1_p = self.get_group_possibles(&col1);
        let col2_p = self.get_group_possibles(&col2);
        let col3_p = self.get_group_possibles(&col3);

        // done finding everything, now mark the affected boxes
        self.mark_list_impossible_outside_subgroup(BigBox::get_row_indices(&row1[0]),
            row1, BigBox::main_vs_sub_unique_possibilities(&row1_p, &row2_p, &row3_p));
        self.mark_list_impossible_outside_subgroup(BigBox::get_row_indices(&row2[0]),
            row2, BigBox::main_vs_sub_unique_possibilities(&row2_p, &row1_p, &row3_p));
        self.mark_list_impossible_outside_subgroup(BigBox::get_row_indices(&row3[0]),
            row3, BigBox::main_vs_sub_unique_possibilities(&row3_p, &row2_p, &row1_p));

        self.mark_list_impossible_outside_subgroup(BigBox::get_col_indices(&col1[0]),
            col1, BigBox::main_vs_sub_unique_possibilities(&col1_p, &col2_p, &col3_p));
        self.mark_list_impossible_outside_subgroup(BigBox::get_col_indices(&col2[0]),
            col2, BigBox::main_vs_sub_unique_possibilities(&col2_p, &col1_p, &col3_p));
        self.mark_list_impossible_outside_subgroup(BigBox::get_col_indices(&col3[0]),
            col3, BigBox::main_vs_sub_unique_possibilities(&col3_p, &col2_p, &col1_p));
    }

    // mark impossible possibilities not in a subgroup
    pub fn mark_list_impossible_outside_subgroup(&mut self, big_group: Vec<u8>, small_group: Vec<u8>, mark_list: Vec<u8>){
        let mut updated = false;
        if mark_list.len() > 0 {
            for index in big_group{
                if !small_group.contains(&index){
                    for number in &mark_list{
                        let was_updated = self.little_boxes[index as usize].make_impossible(*number);
                        if was_updated == true {
                            updated = true;
                        }
                    }
                }
            }
        }

        // update times updated
        if updated == true {
            self.set_times_updated_plus_one();
        }
    }

    // find if main has any unique possibilities that sub1 or sub2 don't have
    pub fn main_vs_sub_unique_possibilities(main: &Vec<u8>, sub1: &Vec<u8>, sub2: &Vec<u8>) -> Vec<u8>{
        let mut row_col_unique: Vec<u8> = Vec::new();
        
        for possibility in main {
            if !sub1.contains(&possibility){
                if !sub2.contains(&possibility){
                    row_col_unique.push(*possibility);
                }
            }
        }

        row_col_unique
    }

    // get possibles of a group of little boxes
    pub fn get_group_possibles(&self, little_box_group: &Vec<u8>) -> Vec<u8> {
        let mut return_vec: Vec<u8> = Vec::new();
        for little_box in little_box_group {
            for possible in self.little_boxes[*little_box as usize].get_possibles(){
                if !return_vec.contains(&possible){
                    return_vec.push(possible);
                }
            }
        }
        return_vec
    }

    // get a Vec<Vec<u8>> of all groups indexes
    pub fn get_all_groups_indexes() -> Vec<Vec<u8>> {
        let mut all_groups_indexes: Vec<Vec<u8>> = Vec::new();

        for index in ROW_STARTERS.iter() {
            all_groups_indexes.push(BigBox::get_row_indices(index));
        }
        for index in COL_STARTERS.iter() {
            all_groups_indexes.push(BigBox::get_col_indices(index));
        }
        for index in BOX_STARTERS.iter() {
            all_groups_indexes.push(BigBox::get_box_indices(index));
        }

        all_groups_indexes
    }

    // this is a catch all for updating notes based on pairs/triples/etc
    pub fn process_and_update_notes(&mut self){
        let all_groups = BigBox::get_all_groups_indexes();
        // this'll be processed at the end
        // in format <all_groups_index, affected_indexes, valid_notes>
        let mut process_list: Vec<(u8, Vec<u8>, Vec<u8>)> = Vec::new();

        for (all_groups_index, this_groups_indexes) in all_groups.iter().enumerate() {
            // gets a list of all notes and their possibilities number
            // format <note, number of times seen>
            let mut group_posibilities_no: HashMap<u8, u8> = HashMap::new();

            // now go through all of this_groups_indexes
            for cell in this_groups_indexes {
                for possibility in self.little_boxes[*cell as usize].get_possibles() {
                    let count = group_posibilities_no.entry(possibility).or_insert(0);
                    *count += 1;
                }
            }

            // got a list of notes and their number of possibilities, now it's time to 
            // process these to see if there are doubles, triples, or even quadruples
            match BigBox::find_valid_multiples(2, &group_posibilities_no) {
                Some(i) => {
                    match self.find_valid_indexes(&this_groups_indexes, &i, 2) {
                        Some(n) => {
                            process_list.push((all_groups_index as u8, n, i));
                        },
                        None => {}
                    }
                },
                None => {}
            }
        }

        let mut note_updated = false;

        for (all_groups_index, affected_indexes, valid_notes) in process_list {
            for cell in &all_groups[all_groups_index as usize] {
                if affected_indexes.contains(cell){
                    for note in ALL_NOTES_POSSIBILITIES.iter() {
                        if !valid_notes.contains(note){
                            if self.little_boxes[*cell as usize].make_impossible(*note){
                                note_updated = true;
                            }
                        }
                    }
                } else {
                    for note in ALL_NOTES_POSSIBILITIES.iter() {
                        if valid_notes.contains(note){
                            if self.little_boxes[*cell as usize].make_impossible(*note) {
                                note_updated = true;
                            }
                        }
                    }
                }
            }
        }

        if note_updated {
            self.set_times_updated_plus_one();
        }
    }

    pub fn find_valid_multiples(type_of_multiple: u8, group_posibilities_no: &HashMap<u8, u8>) -> Option<Vec<u8>> {
        let mut return_notes: Vec<u8> = Vec::new();
        for (key, no_of_possibilities) in group_posibilities_no {
            if no_of_possibilities == &type_of_multiple {
                return_notes.push(*key);
            }
        }

        if return_notes.len() == type_of_multiple as usize {
            Some(return_notes)
        } else {
            None
        }
    }

    // take self, a group I'm evaluating, and the notes in question in format
    // <self, indexes, notes>
    pub fn find_valid_indexes(&self, this_groups_indexes: &Vec<u8>, notes: &Vec<u8>, type_of_multiple: u8)
     -> Option<Vec<u8>> {
        let mut return_indexes: Vec<u8> = Vec::new();
        for cell in this_groups_indexes {
            let cells_possibilities = self.little_boxes[*cell as usize].get_possibles();
            if cells_possibilities.len() > 0 {
                let mut possibility_count = 0;
                for possibility in &cells_possibilities {
                    if notes.contains(&possibility){
                        possibility_count += 1;
                    }
                }

                if possibility_count == notes.len() {
                    return_indexes.push(*cell);
                }
            }
        }

        if return_indexes.len() == type_of_multiple as usize {
            Some(return_indexes)
        } else {
            None
        }
    }

    pub fn process_obvious_pairs(&mut self){
        let all_groups_indexes = BigBox::get_all_groups_indexes();
        // get all groups and start running through them to see if there are pairs of doubles

        // this is a vector for a final run. it keeps an index of all_groups_indexes
        // and the valid pair that can be run on the big board's subgroup.
        let mut valid_obvious_pair_and_group: Vec<(u8, Vec<u8>)> = Vec::new();

        for (index, group) in all_groups_indexes.iter().enumerate(){
            let mut hash_count: HashMap<Vec<u8>, u8> = HashMap::new();

            // go through the cells in the group, add them to the hashmap
            for cell in group {
                let possibilities = self.little_boxes[*cell as usize].get_possibles();
                if possibilities.len() == 2 {
                    let entry = hash_count.entry(possibilities).or_insert(0);
                    *entry += 1;
                }
            }

            // now each cell with two possibilities is in the hashmap, now look for a pair
            for (key, value) in hash_count {
                if value == 2 {
                    valid_obvious_pair_and_group.push((index as u8, key));
                }
            }
        }

        for (index, valid_pair) in valid_obvious_pair_and_group.iter() {
            for cell in &all_groups_indexes[*index as usize] {
                if *valid_pair != self.little_boxes[*cell as usize].get_possibles() {
                    if self.little_boxes[*cell as usize].make_many_impossible(valid_pair.to_vec()) {
                        self.set_times_updated_plus_one();
                    }
                }
            }
        }
    }

    // update with the scan_one_possible list
    pub fn add_valid_values(&mut self, valid_values: Vec<(u8, u8)>){
        for (index, number) in valid_values {
            self.update_little_box_value(&index, &number);
            
            // now update the affected rows
            let affected_list = BigBox::get_all_affected_indices(index);
            for i in affected_list {
                self.little_boxes[i as usize].make_impossible(number);
            }
        }
    }

    pub fn solve_puzzle(&mut self){
        self.mark_impossible(self.scan_for_make_impossible());

        loop {
            let update_list = self.scan_one_possible();
            let total_times_updated = self.get_times_updated();
            if update_list.len() > 0 {
                self.add_valid_values(update_list);
            }

            for i in 0..=80{
                if self.little_boxes[i].get_value() == 0{
                    self.scan_for_one_possible_in_group(i as u8);
                }
            }

            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_1);
            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_2);
            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_3);
            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_4);
            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_5);
            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_6);
            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_7);
            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_8);
            self.find_medium_box_row_col_unique_possibles(MEDIUM_BOX_9);

            

            // this should happen if the board wasn't updated.
            if total_times_updated == self.get_times_updated() {
                self.process_and_update_notes();
            }

            // this should happen if the previous notes check failed
            if total_times_updated == self.get_times_updated() {
                self.process_obvious_pairs();
            }

            // check again if notes weren't updated
            if total_times_updated == self.get_times_updated() {
                break;
            }
        }
    }

    pub fn print_notes(&self) -> String{
        let mut notes: String = "".to_string();
        notes = notes + &"Some notes:<br />".to_string();
        for i in 0..=80 {
            if self.little_boxes[i].get_value() == 0 {
                notes = notes + &format!("Index # {} possibilities: {:?}<br />", i, self.little_boxes[i].get_possibles());
            }
        }
        notes
    }
}

impl WebForm{
    pub fn convert_to_big_box(&self) -> Option<BigBox> {
        BigBox::new(vec!(
            WebForm::convert_cell_to_u8(&self.cell_0),  WebForm::convert_cell_to_u8(&self.cell_1),
            WebForm::convert_cell_to_u8(&self.cell_2),  WebForm::convert_cell_to_u8(&self.cell_3),
            WebForm::convert_cell_to_u8(&self.cell_4),  WebForm::convert_cell_to_u8(&self.cell_5), 
            WebForm::convert_cell_to_u8(&self.cell_6),  WebForm::convert_cell_to_u8(&self.cell_7),
            WebForm::convert_cell_to_u8(&self.cell_8),  WebForm::convert_cell_to_u8(&self.cell_9),
            WebForm::convert_cell_to_u8(&self.cell_10), WebForm::convert_cell_to_u8(&self.cell_11),
            WebForm::convert_cell_to_u8(&self.cell_12), WebForm::convert_cell_to_u8(&self.cell_13),
            WebForm::convert_cell_to_u8(&self.cell_14), WebForm::convert_cell_to_u8(&self.cell_15),
            WebForm::convert_cell_to_u8(&self.cell_16), WebForm::convert_cell_to_u8(&self.cell_17),
            WebForm::convert_cell_to_u8(&self.cell_18), WebForm::convert_cell_to_u8(&self.cell_19),
            WebForm::convert_cell_to_u8(&self.cell_20), WebForm::convert_cell_to_u8(&self.cell_21),
            WebForm::convert_cell_to_u8(&self.cell_22), WebForm::convert_cell_to_u8(&self.cell_23),
            WebForm::convert_cell_to_u8(&self.cell_24), WebForm::convert_cell_to_u8(&self.cell_25),
            WebForm::convert_cell_to_u8(&self.cell_26), WebForm::convert_cell_to_u8(&self.cell_27),
            WebForm::convert_cell_to_u8(&self.cell_28), WebForm::convert_cell_to_u8(&self.cell_29),
            WebForm::convert_cell_to_u8(&self.cell_30), WebForm::convert_cell_to_u8(&self.cell_31),
            WebForm::convert_cell_to_u8(&self.cell_32), WebForm::convert_cell_to_u8(&self.cell_33), 
            WebForm::convert_cell_to_u8(&self.cell_34), WebForm::convert_cell_to_u8(&self.cell_35),
            WebForm::convert_cell_to_u8(&self.cell_36), WebForm::convert_cell_to_u8(&self.cell_37),
            WebForm::convert_cell_to_u8(&self.cell_38), WebForm::convert_cell_to_u8(&self.cell_39),
            WebForm::convert_cell_to_u8(&self.cell_40), WebForm::convert_cell_to_u8(&self.cell_41),
            WebForm::convert_cell_to_u8(&self.cell_42), WebForm::convert_cell_to_u8(&self.cell_43),
            WebForm::convert_cell_to_u8(&self.cell_44), WebForm::convert_cell_to_u8(&self.cell_45),
            WebForm::convert_cell_to_u8(&self.cell_46), WebForm::convert_cell_to_u8(&self.cell_47), 
            WebForm::convert_cell_to_u8(&self.cell_48), WebForm::convert_cell_to_u8(&self.cell_49),
            WebForm::convert_cell_to_u8(&self.cell_50), WebForm::convert_cell_to_u8(&self.cell_51),
            WebForm::convert_cell_to_u8(&self.cell_52), WebForm::convert_cell_to_u8(&self.cell_53),
            WebForm::convert_cell_to_u8(&self.cell_54), WebForm::convert_cell_to_u8(&self.cell_55),
            WebForm::convert_cell_to_u8(&self.cell_56), WebForm::convert_cell_to_u8(&self.cell_57),
            WebForm::convert_cell_to_u8(&self.cell_58), WebForm::convert_cell_to_u8(&self.cell_59),
            WebForm::convert_cell_to_u8(&self.cell_60), WebForm::convert_cell_to_u8(&self.cell_61), 
            WebForm::convert_cell_to_u8(&self.cell_62), WebForm::convert_cell_to_u8(&self.cell_63),
            WebForm::convert_cell_to_u8(&self.cell_64), WebForm::convert_cell_to_u8(&self.cell_65),
            WebForm::convert_cell_to_u8(&self.cell_66), WebForm::convert_cell_to_u8(&self.cell_67),
            WebForm::convert_cell_to_u8(&self.cell_68), WebForm::convert_cell_to_u8(&self.cell_69),
            WebForm::convert_cell_to_u8(&self.cell_70), WebForm::convert_cell_to_u8(&self.cell_71),
            WebForm::convert_cell_to_u8(&self.cell_72), WebForm::convert_cell_to_u8(&self.cell_73),
            WebForm::convert_cell_to_u8(&self.cell_74), WebForm::convert_cell_to_u8(&self.cell_75), 
            WebForm::convert_cell_to_u8(&self.cell_76), WebForm::convert_cell_to_u8(&self.cell_77),
            WebForm::convert_cell_to_u8(&self.cell_78), WebForm::convert_cell_to_u8(&self.cell_79),
            WebForm::convert_cell_to_u8(&self.cell_80)))
    }

    // convert all web form cells to a u8. default to 0 if something is entered incorrectly
    pub fn convert_cell_to_u8(value: &String) -> u8 {
        match value.parse::<u8>() {
            Ok(i) => {
                match i < 10 {
                    true => i,
                    _ => 0
                }
            },
            _ => {
                0 as u8
            }
        }
    }
}