fn main() {
    cxx_build::bridge("src/cxx.rs") // Rust <-> C++ バインディングの定義ファイル
        .file("source/example.cpp") // C++関数の実装ファイル
        .include("source") // ヘッダーファイル格納ディレクトリ
        .flag_if_supported("-std=c++20") // コンパイルオプション(C++20準拠ビルド)
        .compile("rust_cxx"); // 出力されるライブラリ名

    // これらのファイルが変更されたときに再ビルドされるように指定
    println!("cargo:rerun-if-changed=source/example.h");
    println!("cargo:rerun-if-changed=source/example.cpp");
    println!("cargo:rerun-if-changed=src/cxx.rs");
}
