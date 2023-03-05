# png-metadata

Simple PNG text metadata reader that can read from local file and URL.

![image](https://user-images.githubusercontent.com/76484203/222964363-eb552e32-80b8-41d1-a28d-d7a505aa19fb.png)

# Why / What's this for?

Sometimes, I just want to get generation parameters from an image and most of the time, [exif.tools](https://exif.tools) work just fine  
**BUT** if you want access to [pixiv image](https://pixiv.net), ***you can't***  
because pixiv image server ([https://i.pximg.net](https://i.pximg.net)) required `referer` - `https://pixiv.net` header

# Installation

1. Build from source or download (Windows) pre-built from [releases](https://github.com/Meonako/png-metadata/releases)
2. Run executable or pass the argument to the executable

# Usage

- Prefix with `file:` will search for file you specify
  > (e.g. `file:C:/img/example.png`, `file:"C:/secret-img/secret.png"`)
- Prefix with `dir:` will read every `.png` files in the specify directory **EXCLUDES** subfolders  
  > (e.g. `dir:C:/Pictures/AI generated`, `dir:my image/background collection`)
- Prefix with `http` will download the image and read from that image  
  > (e.g `https://mywebserver/example.png`)
- `clear` | `cls` to clear terminal screen
- `quit` | `stop` to exit properly

---

- You can pass in multiple targets  
  e.g.
    - `Command`: `file:1.png, file:2.png` so if file name contains `,`, it'll not work!
    - `Arguments`: `file:1.png file:2.png`
- Apps will try to get text out as much as possible. (checkout `src/utils.rs:get_avaiable_text`)

# Issues

- Twitter images (`pbs.twimg.com`)  
  > I don't have plan to support that because I don't know how to make a request to it  
  but I found the `x-cache` header so it might required additional lib as it may has to do with HTTP caching  
  AND I DON'T WANT THAT

# TODO

- `search:` read every `.png` files in the specify directory includes subfolder

# Credits
  - [png](https://github.com/image-rs/image-png) - Licensed under either of
    - Apache License, Version 2.0 (http://apache.org/licenses/LICENSE-2.0)
    - MIT license (http://opensource.org/licenses/MIT)
    ---
  - [reqwest](https://github.com/seanmonstar/reqwest) - Licensed under either of
    - Apache License, Version 2.0 (http://apache.org/licenses/LICENSE-2.0)
    - MIT license (http://opensource.org/licenses/MIT)
    ---
  - [colored](https://github.com/mackwic/colored) - Mozilla Public License 2.0 (https://www.mozilla.org/en-US/MPL/2.0/)
  
    ---
  - [clap](https://github.com/clap-rs/clap) - Dual-licensed under Apache 2.0 or MIT.
    - Apache License, Version 2.0 (http://apache.org/licenses/LICENSE-2.0)
    - MIT license (http://opensource.org/licenses/MIT)
