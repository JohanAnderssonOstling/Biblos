<script lang="ts">
import {getContext, onMount} from "svelte";
import {invoke} from "@tauri-apps/api/core";
import BookCover from "../components/BookCover.svelte";

const { push , pop } = getContext("stackview");
let loading = true
export let libPath          : string
export let currentPath      : string
interface Dir {path: string, book_paths: string}
let books: string[]         = [];
let dirs: Dir[]          = [];
onMount(() => {load_dir(libPath)})

function back() {
    console.log(currentPath)
    if (currentPath === libPath) {pop(); return;}
    currentPath = currentPath.substring(0, currentPath.lastIndexOf("/"))
    load_dir(currentPath)
}

function load_dir(path: string) {
console.log(currentPath)
    loading = true
    invoke("get_library", {path: path} ).then((bookss) => {
        books = bookss[0];      dirs = bookss[1];       loading = false
    })
}

function openDir(path: string) {
    currentPath = path;     load_dir(currentPath)
}
</script>

<div class="thing">

    <div class = "topbar">
        <button on:click={back}>Back</button>
        <input type = "text" placeholder= "Search..."/>
    </div>

    {#if loading} <h1>Loading</h1>

    {:else}
        <h1>{currentPath.split("/").slice(-1)}</h1>
    <div class = "bookgrid">
        {#each books as book}
            <BookCover book_path={book} libPath={libPath}></BookCover>
        {/each}
    </div>

    {#each dirs as dir}
        <div class = "directory">
            <h2 on:click={() => openDir(dir.path)}>
                <a href="#">
                {dir.path.split("/").slice(-1)}
                </a>
            </h2>
            <div class = "bookgrid">
                {#each dir.book_paths as book}
                    <BookCover book_path={book} libPath={libPath}></BookCover>
                {/each}
            </div>
        </div>
    {/each}
    {/if}
</div>

<style>
    .thing {
        justify-content: center;
    }
    .bookgrid {
        justify-content: center;
        display: grid;
        grid-gap: 20px;
        grid-template-columns: repeat(auto-fill, 200px);
    }

    h2 {
        text-align: center;
    }
</style>