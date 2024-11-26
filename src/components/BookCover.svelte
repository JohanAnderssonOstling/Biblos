<script lang="ts">
    import {getContext, onMount} from "svelte";
    import BookReader from "../pages/BookReader.svelte";
	import {invoke} from "@tauri-apps/api/core"

	interface Book  {title: string; path:   string; cover:  string;}
    export let book_path: string
    let book: Book
    let loading = true
    export let libPath: string
    const { push ,pop} = getContext("stackview");

    onMount(() => {
	    invoke("get_book_cover", {path: book_path}).then((cover) => {
            book = cover    ;loading = false
        })
        console.log("hello")
    })

    function openBook(path: string) {
        push(BookReader, {bookPath: path, libraryPath: libPath})
    }
</script>

<div class = "bookgrid-item" >
    {#if loading}
        <h1>Loading</h1>
    {:else}
    <img class = "cover" src="{book.cover}" on:click={() => openBook(book.path)}>
    <span> {book.title}</span>
    {/if}
</div>


<style>
    .bookgrid-item {
        display: flex;
        flex-direction: column;
        text-align: center;
        justify-content: flex-end;
        height: 350px;
        width: 200px;
    }

    .cover:hover {
        box-shadow: 4px 4px 16px rgba(0, 0, 0, 0.8);
        background: rgba(0,0,0,0.3);
    }
    .cover {
        width: 200px;
        height: 300px;
        border: 1px solid #aaa; border-radius: 4px;
        box-shadow: 4px 4px 16px rgba(0, 0, 0, 0.5);
        cursor: pointer;
    }

    span {
        font-family: serif;
        height: 3em;
        overflow-y: clip;
        overflow-block: hidden;
        max-lines: 2;
    }
</style>