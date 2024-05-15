## How to Run

Require to set database connection `src-tauri/.env` or you can run postgres database from this [kanoon/go-gin-gorm-api](ttps://github.com/kanoon/go-gin-gorm-api) <br />
https://github.com/kanoon/go-gin-gorm-api/tree/master?tab=readme-ov-file#run-db-and-api-with-docker-container

```
npx tuari dev
```

## How to build exe file

Finished 2 bundles at:<br />

- `tauri-app\src-tauri\target\release\bundle\msi\tauri-app_0.0.0_x64_en-US.msi`
- `tauri-app\src-tauri\target\release\bundle\nsis\tauri-app_0.0.0_x64-setup.exe`

Require: Please manual copy your `src-tauri/.env` to your installed folder.

```
npm run tauri build
```
