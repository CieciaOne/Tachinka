const invoke = window.__TAURI__.invoke

export async function invokeHello(name) {
	return await invoke("hello", { name: name });
}

export async function invokeGetMangaList(tags, offset) {
	return await invoke("get_manga_list", { tags, offset });
}