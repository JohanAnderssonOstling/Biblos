<script lang="ts">
    import {getContext, onMount} from "svelte";
    import {dialog, tauri} from "@tauri-apps/api";
    import Lib from "./Library.svelte";
    import BookCover from "../components/BookCover.svelte";
	import Carousel from "../components/Carousel.svelte";
    interface Lib {path: string, book_paths: string}
    let libs: Lib[] = [];
    const { push , pop} = getContext("stackview");

    onMount(() => {getLibs()});
    function getLibs() {
        tauri.invoke("get_libraries").then((lib) => {libs = lib})
    }
    function pushLib(path: string) {push(Lib, {libPath: path, currentPath: path})}

    async function addLibrary() {
        const path: string = await dialog.open({directory: true, multiple: true});
        tauri.invoke("create_libraries", {paths: path}).then(() => {getLibs()})
    }
    function deleteLibrary(lib: string) {
        tauri.invoke("delete_library", {libraryPath: lib}).then(() => {getLibs()})
    }
</script>
    <button on:click={addLibrary}>Add Library</button>
    <div class = "library-list">
        {#each libs as lib}
            <div class="library" >
                <div class = "button-row">
                    <h1 on:click={() => pushLib(lib.path)}>
                        {lib.path.split("/").slice(-1)}
                    </h1>
                </div>

                <div class = "last-read">
                    {#each lib.book_paths as book}
                        <BookCover book_path={book} libPath={lib.path}></BookCover>
                    {/each}
                </div>
            </div>
        {/each}
    </div>

<style>
    .library-list {
        display: grid;
        gap: 20px;
        grid-template-columns: repeat(auto-fit, minmax(680px, 1fr));
        justify-content: center;
        margin: 20px;
    }
    .last-read {
        gap: 20px;
        display: flex;
        align-items: center;
        overflow-y: hidden;
        overflow-x: auto;
    }
    .library {
        border: 1px solid #aaa; border-radius: 2px;
        box-shadow: 2px 2px 8px rgba(0, 0, 0, 0.1);
        height: 420px;
        padding: 20px;
    }

    .button-row {
        justify-content: center;
        display: flex;
    }

    h1 {
        font-family: "sans-serif";
        font-weight: normal;
    }
</style>