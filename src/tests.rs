use super::sort;
use super::sort_by;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::str;

#[test]
fn test_sort_in_buf() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(
        fin,
        "寿限無、寿限無、
五劫の擦り切れ、
海砂利水魚の、
水行末・雲来末・風来末、
喰う寝る処に住む処、
藪ら柑子の藪柑子、
パイポ・パイポ・パイポのシューリンガン、
シューリンガンのグーリンダイ、
グーリンダイのポンポコピーのポンポコナの、
長久命の長助
"
    )
    .unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort(fin, fout, 1024).unwrap();

    assert_eq!(
        "グーリンダイのポンポコピーのポンポコナの、
シューリンガンのグーリンダイ、
パイポ・パイポ・パイポのシューリンガン、
五劫の擦り切れ、
喰う寝る処に住む処、
寿限無、寿限無、
水行末・雲来末・風来末、
海砂利水魚の、
藪ら柑子の藪柑子、
長久命の長助
",
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_using_file() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(
        fin,
        "寿限無、寿限無、
五劫の擦り切れ、
海砂利水魚の、
水行末・雲来末・風来末、
喰う寝る処に住む処、
藪ら柑子の藪柑子、
パイポ・パイポ・パイポのシューリンガン、
シューリンガンのグーリンダイ、
グーリンダイのポンポコピーのポンポコナの、
長久命の長助
"
    )
    .unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort(fin, fout, 50).unwrap();

    assert_eq!(
        "グーリンダイのポンポコピーのポンポコナの、
シューリンガンのグーリンダイ、
パイポ・パイポ・パイポのシューリンガン、
五劫の擦り切れ、
喰う寝る処に住む処、
寿限無、寿限無、
水行末・雲来末・風来末、
海砂利水魚の、
藪ら柑子の藪柑子、
長久命の長助
",
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_empty() {
    let fin = tempfile::tempfile().unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort(fin, fout, 50).unwrap();

    assert_eq!("", str::from_utf8(&buf).unwrap());
}

#[test]
fn test_sort_one_line() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "寿限無、寿限無、\n").unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort(fin, fout, 50).unwrap();

    assert_eq!("寿限無、寿限無、\n", str::from_utf8(&buf).unwrap());
}

#[test]
fn test_sort_two_lines() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "寿限無、寿限無、\n五劫の擦り切れ、\n").unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort(fin, fout, 50).unwrap();

    assert_eq!(
        "五劫の擦り切れ、\n寿限無、寿限無、\n",
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_three_lines() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "寿限無、寿限無、\n五劫の擦り切れ、\n海砂利水魚の、\n").unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort(fin, fout, 50).unwrap();

    assert_eq!(
        "五劫の擦り切れ、\n寿限無、寿限無、\n海砂利水魚の、\n",
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_desc() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(
        fin,
        "寿限無、寿限無、
五劫の擦り切れ、
海砂利水魚の、
水行末・雲来末・風来末、
喰う寝る処に住む処、
藪ら柑子の藪柑子、
パイポ・パイポ・パイポのシューリンガン、
シューリンガンのグーリンダイ、
グーリンダイのポンポコピーのポンポコナの、
長久命の長助
"
    )
    .unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    let cmp = |a: &String, b: &String| {
        let a = a.trim_end_matches(|c| c == '\r' || c == '\n');
        let b = b.trim_end_matches(|c| c == '\r' || c == '\n');
        b.partial_cmp(a).unwrap()
    };

    sort_by(fin, fout, 50, cmp).unwrap();

    assert_eq!(
        "長久命の長助
藪ら柑子の藪柑子、
海砂利水魚の、
水行末・雲来末・風来末、
寿限無、寿限無、
喰う寝る処に住む処、
五劫の擦り切れ、
パイポ・パイポ・パイポのシューリンガン、
シューリンガンのグーリンダイ、
グーリンダイのポンポコピーのポンポコナの、
",
        str::from_utf8(&buf).unwrap()
    );
}
