#! /bin/sh

set -e

mkdir -p input src

year=2021
day=$1

if [ "$day" = "" ]; then
    day=$(TZ=US/Eastern date '+%d' | sed 's/^0//')
fi

input="input/day$day.txt"
src="src/day$day.rs"

if ! git diff --exit-code > /dev/null; then
    echo "There are uncommitted changes" 2>&1
    exit 1
elif [ -e "$input" ]; then
    echo "Already have day $day" 2>&1
    exit 1
fi

# download input (assumes w3m is already logged in)
url="https://adventofcode.com/$year/day/$day/input"
echo "Fetching $url..."
w3m "$url" > "$input"
file "$input"

echo "Creating $src..."
cat > "$src" <<EOF
use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    // let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => "?",
            Part::Two => "?",
        }
    )
}

#[test]
fn test() {
    let test_input = "\
";
    // assert_eq!()
}
EOF

echo "Preview of input:"
head -c 200 "$input" | head -10
