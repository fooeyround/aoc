use core::str;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}
pub fn parse_input(raw_input: &str) -> Vec<u8> {
    strip_trailing_newline(raw_input.trim())
        .chars()
        .map(|f| f.to_digit(10).expect("Number") as u8)
        .collect()
}

pub fn solve1(raw_input: &str) -> String {
    let mut input = parse_input(raw_input);

    let mut sum = 0;

    let mut final_pos = 0;

    'outer: for id in 0..input.len() {
        if id % 2 == 0 {
            let summed: usize = (final_pos..(final_pos + input[id] as usize)).sum();
            sum += summed * (id / 2);
            final_pos += input[id] as usize;
        } else {
            for _ in 0..input[id] {
                let Some((id, _)) = input
                    .iter()
                    .enumerate()
                    .rfind(|(iid, val)| iid % 2 == 0 && **val != 0 && *iid > id)
                else {
                    break 'outer;
                };
                input[id] -= 1;
                sum += final_pos * (id / 2);
                final_pos += 1;
            }
        }
    }
    return sum.to_string();
}

#[derive(Debug)]
enum File {
    FreeSpace(u8),
    File(usize, u8),
}

fn input_to_file(input: Vec<u8>) -> Vec<File> {
    input
        .iter()
        .enumerate()
        .map(|(id, val)| {
            if id % 2 == 0 {
                File::File(id / 2, *val)
            } else {
                File::FreeSpace(*val)
            }
        })
        .filter(|file| match file {
            File::File(_, _) => true,
            File::FreeSpace(size) => *size > 0,
        })
        .collect()
}

pub fn solve2(raw_input: &str) -> String {
    let mut input = input_to_file(parse_input(raw_input));

    let mut rev_idx = 0;
    while rev_idx < input.len() {
        let id = input.len() - rev_idx - 1;

        let file = match input[id] {
            File::File(id, val) => (id, val),
            _ => {
                rev_idx += 1;
                continue;
            }
        };

        let Some(fitting_free_space) = (0..id).find(|&iid| match input[iid] {
            File::File(_, _) => false,
            File::FreeSpace(size) => size >= file.1,
        }) else {
            rev_idx += 1;
            continue;
        };

        input[id] = File::FreeSpace(file.1);

        let freefile = &input[fitting_free_space];

        if let File::FreeSpace(size) = freefile {
            if *size == file.1 {
                input.remove(fitting_free_space);
            } else {
                input[fitting_free_space] = File::FreeSpace(size - file.1)
            }
            input.insert(fitting_free_space, File::File(file.0, file.1));
        }

        rev_idx += 1;
    }

    let mut sum = 0;
    let mut final_index = 0;

    for file in input.iter() {
        match file {
            File::File(fid, size) => {
                let summed: usize = (final_index..(final_index + *size as usize)).sum();
                sum += summed * (fid);
                final_index += *size as usize;
            }
            File::FreeSpace(size) => {
                final_index += *size as usize;
            }
        };
    }
    return sum.to_string();
}
