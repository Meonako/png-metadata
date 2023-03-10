# png-metadata (Outdated README. I'll update this later)

Simple PNG text metadata reader that can read from local file and URL.

![image](https://user-images.githubusercontent.com/76484203/224237863-950ccbc8-15cd-47a1-a769-fb099ffb7ae2.png)

# Why / What's this for?

Sometimes, I just want to get generation parameters from an image and most of the time, [exif.tools](https://exif.tools) work just fine  
**BUT** if you want to read PNG metadata from [pixiv](https://pixiv.net), ***you can't***  
because pixiv image server ([https://i.pximg.net](https://i.pximg.net)) required `referer` - `https://pixiv.net` header

# Installation

1. Build from source or download Windows pre-built from [releases](https://github.com/Meonako/png-metadata/releases)
2. Run executable or pass the argument to the executable

# Usage

- Prefix with `all:` will read every `.png` files in the specify directory **INCLUDES** subfolders
  > (e.g. `all:image collection`, `all: "scan these subfolders"`)
- Prefix with `http` will download the image and read from that image  
  > (e.g `https://mydoma.in/example.png`)
- If not with the 2 above prefix and all commands below, It'll read every `.png` found in directory OR a file
  > (e.g. `C:\Pictures\My Image Collection`, `My Image Collection/example.png`)
- `clear` | `cls` to clear terminal screen
- `quit` | `stop` | `exit` to exit properly
- `update` to check for update

---

- You can pass in multiple targets  
  e.g.
    - `Command`: 
        ```
        1.png, 2.png
        ```
        or 
        ```
        "1, 2.png", "3, 4.png"
        ``` 
        if comma (`,`) is in the file name
    - `Arguments`: 
        ```
        ./metadata 1.png 2.png
        ```
        or
        ```
        ./metadata "this file or dir contains space"
        ```
        if space (` `) is in the file name
- Apps will try to get text out as much as possible. (checkout [`src/utils.rs:get_avaiable_text`](https://github.com/Meonako/png-metadata/blob/master/src/utils.rs#L11))

# Issues

- Twitter images (`pbs.twimg.com`)  
  > I don't have plan to support this because I don't know how to make a request to it  
  but I found the `x-cache` header so it might required additional lib as it may has to do with HTTP caching  
  AND I DON'T WANT THAT

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
