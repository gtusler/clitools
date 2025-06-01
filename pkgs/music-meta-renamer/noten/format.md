# Format

The `format` arg is the most important part of this.

It accepts a string which can contain placeholder values which will be replaced by values from the metadata on the file.

I'm not sure how this should look.


For now, this tool will only work for mp3 files, so the `.mp3` suffix can be omitted.


These are equivalent.
```
"%artist% - %song%"
"%artist% - %song%.mp3"
```

### Parameters

- artist - The Artist name. Default "Unknown Artist".
- album - The Album name. Default "Unknown Album".
- song - The Song name. Default "Unknown Song".
