## コンパイル手順

1. **IR からバイトコード生成**  
    ```sh
    llvm-as main.ll -o main.bc
    ```

2. **オブジェクトファイル生成**  
    ```sh
    llc main.bc -filetype=obj -o main.o
    ```

3. **実行ファイル生成**  
    ```sh
    clang main.o -o main.exe
    ```

    あるいはバイトコードを直接リンク:

    ```sh
    clang main.bc -o main.exe
    ```