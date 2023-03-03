# png-metadata

Simple PNG text metadata reader using [png](https://github.com/image-rs/image-png), [reqwest](https://github.com/seanmonstar/reqwest) and [colored](https://github.com/mackwic/colored) to stylish for more readable outputs

![image](https://user-images.githubusercontent.com/76484203/222798344-e02b7197-cdd0-4309-8230-0544ff73d74d.png)

# Why / What's this for?

Sometimes, I just want to get generation parameters from an image and most of the time, [exif.tools](https://exif.tools) work just fine  
**BUT** if you want access to [pixiv image](https://pixiv.net), ***you can't***  
because pixiv image server ([https://i.pximg.net](https://i.pximg.net)) required `referer` - `https://pixiv.net` header

# Usage

1. Build from source or download Windows pre-built from [releases](https://github.com/Meonako/png-metadata/releases)
2. Run executable

# Credits
  - [png](https://github.com/image-rs/image-png) - Licensed under either of
    - Apache License, Version 2.0 (http://apache.org/licenses/LICENSE-2.0)
    - MIT license (http://opensource.org/licenses/MIT)
  - [reqwest](https://github.com/seanmonstar/reqwest) - Licensed under either of
    - Apache License, Version 2.0 (http://apache.org/licenses/LICENSE-2.0)
    - MIT license (http://opensource.org/licenses/MIT)
  - [colored](https://github.com/mackwic/colored) - Mozilla Public License 2.0 (https://www.mozilla.org/en-US/MPL/2.0/)
