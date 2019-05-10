# flac-tag-split

Some software writes tags of the form `GENRE=Jazz, Modal`, whereas the standard
way is to create to tags:
```
GENRE=Jazz
GENRE=Modal
```

This tool fixes such tags. Example:
```
~/ $ soxi Kind\ of\ Blue/01\ So\ What.flac | grep -i genre
GENRE=Jazz, Modal
~/ $ flac-field-split -d ', ' -f genre -- Kind\ of\ Blue/*flac
~/ $ soxi Kind\ of\ Blue/01\ So\ What.flac | grep -i genre
GENRE=Jazz
GENRE=Modal
~/ $
```
