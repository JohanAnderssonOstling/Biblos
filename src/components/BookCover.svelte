<script lang="ts">
    import {getContext, onMount} from "svelte";
    import BookReader from "../pages/BookReader.svelte";
    import {tauri} from "@tauri-apps/api";
    interface Book  {title: string; path:   string; cover:  string;}
    export let book_path: string
    let book: Book
    let loading = true
    export let libPath: string
    const { push ,pop} = getContext("stackview");

    onMount(() => {
	    tauri.invoke("get_book_cover", {path: book_path}).then((cover) => {
            book = cover    ;loading = false
        })
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
        transition: box-shadow 0.3s ease;
        height: 350px;
        max-width: 200px;
    }

    .cover:hover {
        box-shadow: 0 8px 16px rgba(0, 0, 0, 1);
        background: rgba(0,0,0,0.3);
    }
    .cover {
        width: 200px;
        border-radius: 8px;
        box-shadow: 1px 1px 1px rgba(0, 0, 0, 0.9);
    }

    span {
        font-family: serif;
        height: 3em;
        overflow-y: clip;
        overflow-block: hidden;
        max-lines: 2;
    }
</style>