use std::path::PathBuf;

fn main() {
    let mut first_flag = true;

    for arg in std::env::args() {
        // Argsの最初の1個を飛ばす。プログラム名なので
        if first_flag {
            first_flag = false;
            continue;
        }

        let path = PathBuf::from(arg);
        // println!("FROM: {}", path.to_str().unwrap());

        // 新しいパスを取得する。取得できなかったときは次の引数へ
        let new_path = match new_path(&path) {
            Some(p) => p,
            None => continue,
        };
        // println!("NEW : {}", new_path.to_str().unwrap());

        // ファイル名を変更する。失敗したときだけキャッチ
        if let Err(e) = std::fs::rename(path, new_path) {
            eprintln!("ファイル名の変更に失敗: {:?}", e);
            continue;
        }
    }

    // ウィンドウを残す
    // let mut s = String::new();
    // println!("エンターキーを押すと終了するよ。");
    // std::io::stdin().read_line(&mut s).ok();
}

// 渡されたパスから、記号を追加した新しいパスを生成する。
// 生成できなかったときはNone。
fn new_path(origin: &PathBuf) -> Option<PathBuf> {
    // ディレクトリだったときは対象外
    if origin.is_dir() {
        return None;
    }

    // ファイルが存在しない場合は対象外
    if origin.exists() == false {
        // eprintln!("Not Exists: {:?}", origin);
        return None;
    }

    // 対象のディレクトリを取得する
    let mut target_dir = origin.clone();
    target_dir.pop();
    // eprintln!("{:?}", target_dir);

    // ファイル名を取得する
    let file_name = match origin.file_name() {
        Some(name) => name,
        None => {
            eprintln!("元ファイル名の取得に失敗: {:?}", origin);
            return None;
        }
    };

    // 新しいファイル名を生成する
    let new_name = match file_name.to_str() {
        Some(v) => v,
        None => {
            eprintln!("元ファイル名の変換に失敗: {:?}", file_name);
            return None;
        }
    };
    let mut new_name = format!("!{}", new_name); // 頭に「!」を付与

    // 新しいパスを生成する
    let mut new_path = origin.with_file_name(new_name);

    // 同名のファイルが存在している場合は、末尾にアンダースコアを追加した新しいパスを生成
    while new_path.exists() {
        let name = new_path.file_stem().unwrap().to_str().unwrap();
        let ext = new_path.extension().unwrap().to_str().unwrap();
        new_name = format!("{}_.{}", name, ext); // アンダースコアを追加
        new_path = origin.with_file_name(new_name);
    }

    Some(new_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;
    const TEST_DIR: &str = "test_files";

    #[test]
    fn ディレクトリが渡されたらnoneを返す() {
        let result = new_path(&current_dir().unwrap());
        assert_eq!(result, None);
    }

    #[test]
    fn 存在しないファイルが渡されたらnoneを返す() {
        let mut before = current_dir().unwrap();
        before.push(TEST_DIR);
        before.push("test000.txt");
        let result = new_path(&before);
        assert_eq!(result, None);
    }

    #[test]
    fn 変更後のファイル名が重複しておらずファイル名が変更されるとき() {
        let mut before = current_dir().unwrap();
        before.push(TEST_DIR);
        before.push("test001.txt");
        let mut after = current_dir().unwrap();
        after.push(TEST_DIR);
        after.push("!test001.txt");
        let result = new_path(&before).unwrap();
        assert_eq!(after, result);
    }

    #[test]
    fn 変更後のファイルが1個存在しているとき() {
        let mut before = current_dir().unwrap();
        before.push(TEST_DIR);
        before.push("test002.txt");
        let mut after = current_dir().unwrap();
        after.push(TEST_DIR);
        after.push("!test002_.txt");
        let result = new_path(&before).unwrap();
        assert_eq!(after, result);
    }

    #[test]
    fn 変更後のファイルが複数回存在しているとき() {
        let mut before = current_dir().unwrap();
        before.push(TEST_DIR);
        before.push("test003.txt");
        let mut after = current_dir().unwrap();
        after.push(TEST_DIR);
        after.push("!test003___.txt");
        let result = new_path(&before).unwrap();
        assert_eq!(after, result);
    }
}
