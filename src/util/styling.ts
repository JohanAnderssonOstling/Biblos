import {invoke} from "@tauri-apps/api/core"

export function writeSetting (setings_name: string, setting: string, value: string) {
	const args = {setting: setting, value: value};
	invoke("set_setting", args).then(() => {});
}