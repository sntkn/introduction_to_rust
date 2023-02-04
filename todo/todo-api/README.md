# TODO Sample App

## packages

- hyper: http リクエストを扱うパッケージ
- tower: サーバー構築周りを扱うパッケージ
- serde: json パース系パッケージ
- mime: http ヘッダーの MIME 定義パッケージ
- tracing: ロギング、デバッグ系パッケージ
- anyhow, thiserror: Result を扱いやすくする Utility パッケージ

## Memo

```rust

// + は多重継承
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
}

todo()! // 未実装

// trait(interface) を使って継承する場合
pub trait TodoRepository {
  fn todo(&self) -> String;
}
impl TodoRepositoryForMemory {
  pub title: String;
}
impl TodoRepository for TodoRepositoryForMemory {
  fn todo(&self) -> String {
    self.title
  }
}
```

## TODO

- [ ] ユーザーの識別
- [ ] ログイン機能
- [ ] 日付順のソート機能
- [ ] マークダウン解析し、Todo の詳細情報を保存
