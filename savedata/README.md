`binwalk DunDefHeroes.dun` shows zlib compressed data starting at offet 0x2C.
Manually extracting this data with a hex editor into `DunDefHeroes.zz` and 
decompressing it with `pigz -d -z DunDefHeroes.zz` will result in the decompressed files here.