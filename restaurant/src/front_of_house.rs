// モジュールを使うことで、関連する定義を一つにまとめる
// front_of_houseモジュールの親には暗黙的に作られたcrateモジュールがある
  pub mod hosting {
      pub fn add_to_waitlist() {}

      fn seat_at_table() {}
  }

  mod serving {
      fn take_order() {}

      fn serve_order() {}

      fn take_payment() {}
  }