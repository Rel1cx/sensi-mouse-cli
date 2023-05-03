# mac-mouse-sys

Tiny wrapper for mouse-related functions in macOS IOKit/hidsystem

Definition of terms:

- `pointer resolution` - the resolution of pointer movement space, the higher the value, the slower the pointer movement speed
    Special values:

  - `-1`: null
  - `5 * 65536`: Min
  - `1995 * 65536` Max

- `mouse acceleration` - the acceleration of the pointer movement, the higher the value, the faster the pointer speed will increase, 0 means no acceleration
    Special values:

  - `0`: No acceleration
  - `45056`: macOS default

## Todo

- [x] Add CI
- [x] Add tests
- [x] Add documentation
- [ ] Add examples
- [ ] Add more settings
