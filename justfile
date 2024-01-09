alias ct := c-test
alias api := api-test

run:
  cargo leptos watch

c-test:
  cargo test

test:
  just ct
  just api-test

api-test:
  python3 ./test-data/test.py
