alias ct := c-test
# alias api := api-test
alias tw := tailwind

run:
  cargo leptos watch

c-test:
  cargo test

test:
  just ct
  # just api-test

# api-test:
#   python3 ./test-data/test.py

tailwind:
  npx tailwindcss -i ./input.css -o ./style/output.css --watch
