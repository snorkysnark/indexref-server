# Indexref Server

## Config file

**Location**:
- Linux: `~/.config/indexref-server/config.toml`
- Windows: `%APPDATA%\snorkysnark\Indexref-Server\config\config.toml`
- Mac: `~/Library/Application Support/com.snorkysnark.Indexref-Server\config.toml`

**Example:**
```toml
[sources]
telegram_chat = '/home/lisk/Work/indexref/data/примеры данных/ChatExport'
single_file_z = '/home/lisk/Work/indexref/data/примеры данных/SingleZ'
scrapbook = '/home/lisk/Work/indexref/data/примеры данных/scrapbook'

[server]
port = 3200
```

## Usage:

- `indexref-server index` - Rebuild the index (the old one will be deleted)
- `indexref-server serve` - Start local server

## Server

- `http://localhost:3200/` - Tree visualization (Web)
- `http://localhost:3200/nodes` - List all nodes (JSON)
- `http://localhost:3200/node/<id>` - Expanded node data (JSON)
- `http://localhost:3200/files/<source>/<..path>` - Access a file in one of the source folders\
  (see the `"file_proxy"` JSON field)

***Tip**: In order to view formatted JSON, either use Firefox or Chrome with the
[JSON Viewer](https://chrome.google.com/webstore/detail/json-viewer/gbmdgpbipfallnflgajpaliibnhdgobh) extension*
