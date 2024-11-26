<script lang="ts">
import BookCover from "./BookCover.svelte";
import {ScrollArea} from "$lib/components/ui/scroll-area/index.js";
import {getContext, onMount} from "svelte";
import Book from "./BookCover.svelte";
import {core} from "@tauri-apps/api";
import Library from "../pages/Library.svelte";

const { push ,pop} = getContext("stackview");
interface Dir   {name:  string; books:  Book[]}
export let dir_path: string
export let libraryPath: string



function openDir(path: string) {
    console.log("Opening dir: " + path);
    push(Library, {libraryPaths: libraryPath, currentPath: path})
}
</script>

<h1 on:click={() => openDir(dir_path)}>{getName(dir_path)}</h1>


<style>
    .flex {
        display: flex;
        flex-direction: row; /* Ensures items are in a horizontal line */
        gap: 20px; /* Optional: adds space between items */
    }
</style>