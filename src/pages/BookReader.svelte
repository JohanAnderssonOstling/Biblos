<script lang="ts">
import {getContext, onMount} from "svelte";
    import {tauri} from "@tauri-apps/api";
    import TreeView from "../components/TreeView.svelte";
    import TocElem from "../components/TreeView.svelte"
    import BookParser from "../util/book_parser"
const { push ,pop} = getContext("stackview");

    export let bookPath     : string
    export let libraryPath  : string

    let renderer                = document.createElement("div");
    let elems                   = document.createElement("div");
    let contents: string[]      = []
    let toc: TocElem[]          = []
    let imgDict                 = {}
    let links                   = {}
    let section_hrefs           = {}
    let hash                    = 0
    let index                   = 0;
    let elemStartIndex          = 0
    let elemEndIndex            = 0;
    let font_size               = 26
    let font                    = "serif"
    let showSearch              = false
    let showRightSideBar        = false

    onMount(() => {
        //document.addEventListener("click", handleLinkClick)
        let args = {bookPath: bookPath, libraryPath: libraryPath}
        tauri.invoke("get_book_content", args).then((cont) => {
            contents        = cont.contents
            index           = cont.section_index
            elemStartIndex  = cont.elem_index
            elemEndIndex    = cont.elem_index
            hash            = cont.hash
            toc             = cont.toc
            let hrefs       = cont.section_hrefs
            cont.style_sheets.forEach(apply_css)
            for (let i = 0; i < hrefs.length; i++) {section_hrefs[hrefs[i]] = i}
            add_content(contents[index])
            renderForward()
            setTimeout(() => {loadImages()}, 5);
        }).catch(error => {console.error("Err: ", error)})
    });

	function apply_css(css: string) {
	const style = document.createElement('style');
	style.textContent = css;
	document.head.appendChild(style);
	renderer.style.fontSize = "26px"
    }

    function loadImages() {
        tauri.invoke("get_images", {bookPath: bookPath}).then((imgs) => {
            for (let i = 0; i < imgs.length; i++) {imgDict[imgs[i][0]] = imgs[i][1]}
        })
    }

    function apply_style(elem: HTMLElement) {
        //elem.style.fontSize     = font_size.toString() + "px"
        //elem.style.width        = (font_size * 20).toString() + "px"
        elem.style.fontFamily   = font
        elem.style.color = "#3c3836"
        if (elem.tagName == "A") elem.style.color = "#076678"
        for (let child of elem.children) {apply_style(child as HTMLElement)}
    }

    function add_content(content: string) {
        links = {}
        let temp_div = document.createElement("div")
        temp_div.innerHTML = content;
        elems.innerHTML = ""
        let book_parser = new BookParser(links, imgDict, elems)
        for (let i = 0; i < temp_div.children.length; i++) {
            let child = (temp_div.children[i] as HTMLElement)
            apply_style(child)
            book_parser.parse_elem(child)
        }
    }

    function writePosition() {
        const args = {libPath: libraryPath,hash: hash, sectionIndex: index, elemIndex: elemStartIndex}
        tauri.invoke("write_book_position", args).then(() => {})
    }

    function renderForward() {
        let child = elems.children[elemEndIndex].cloneNode(true) as HTMLElement
        renderer.appendChild(child)
        if (elemEndIndex >= elems.children.length -1 ) return;
        if (child.getBoundingClientRect().right > renderer.getBoundingClientRect().right) {
            renderer.removeChild(renderer.lastChild)
            elemEndIndex--
            return
        }
        elemEndIndex++;
        renderForward()
    }

    function renderBackward() {
        let child = elems.children[elemStartIndex].cloneNode(true) as HTMLElement
        renderer.prepend(child)
        if (elemStartIndex == 0) return;
        let last = renderer.children[renderer.children.length - 1] as HTMLElement
        if (last.getBoundingClientRect().right > renderer.getBoundingClientRect().right) {
            renderer.removeChild(renderer.firstChild)
            elemStartIndex++
            return
        }
        elemStartIndex--
        renderBackward()
    }

    function next() {
	    if (elemEndIndex == elems.children.length - 1) { return next_section() }
	    renderer.innerHTML = ""
        elemStartIndex = ++elemEndIndex;
        renderForward()
        writePosition()
    }

    function next_section() {
	    if (index == contents.length - 1) return
        renderer.innerHTML = ""
        index++;
        elemEndIndex = 0;
        elemStartIndex = elemEndIndex;
        add_content(contents[index])
        renderForward()
        writePosition()
    }

    function prev() {
	    if (elemStartIndex <= 0) { return prev_chapter(); }
	    renderer.innerHTML = ""
        elemEndIndex = --elemStartIndex;
        renderBackward()
        elemEndIndex++
        renderForward()
        writePosition()
    }

    function prev_chapter() {
	    if (index == 0) return;
	    renderer.innerHTML = ""
        index--;
        add_content(contents[index])
        elemEndIndex = elems.children.length - 1
        elemStartIndex = elemEndIndex
        renderBackward()
        writePosition()
    }

    function handleKeyPress(event: KeyboardEvent) {
        if (event.ctrlKey) {
            switch (event.key) {
                case "ArrowRight"   : next_section(); break
                case "ArrowLeft"    : prev_chapter(); break
                case "s"            : showSearch != showSearch
            }
        }
        else {
            switch (event.key) {
                case "ArrowRight"   : next(); break;
                case "ArrowLeft"    : prev(); break;
                case "+"            : change_font_size(++font_size); break
                case "-"            : change_font_size(--font_size); break
            }
        }
    }

    function change_font_size(size: number) {
        font_size = size
        apply_style(elems)
        renderer.style.columnWidth = (20 * font_size).toString() + "px"
        elemEndIndex = elemStartIndex
        renderer.innerHTML = ""
        renderForward()
    }

    function handleTocItemClick(value: string) {
        if (value.includes("www.")) return
        renderer.innerHTML  = ""
        let section_href    = value.split("#")[0]
        index               = section_hrefs[section_href]
        add_content(contents[index])
        let elem_href       = value.split("#")[1]
        let elemIndex       = 0
        if (elem_href && links[elem_href]) elemIndex = links[elem_href]
        elemStartIndex      = elemIndex
        elemEndIndex        = elemIndex
        renderForward()
        writePosition()
    }

    function handleLinkClick(event: MouseEvent) {
        event.preventDefault()
        const target = event.target as HTMLElement

        if (target.parentNode && target.parentNode.nodeName == "A") {
            handleTocItemClick(target.parentNode.getAttribute("href"))
        }
        else if (target.nodeName == "A") {
            handleTocItemClick(target.getAttribute("href"))
        }
    }

</script>
<div>
    <span>Section Index: {index}</span>
    <span>StartElem Index: {elemStartIndex}</span>
    <span>EndElem Index: {elemEndIndex}</span>
</div>
<button on:click={pop}> Back </button>
<button on:click={() => showRightSideBar = !showRightSideBar}>Sidebar</button>
<div class = "book-reader">
    {#if showRightSideBar}
        <div class="toc">
            <TreeView items={toc} onItemClicked={handleTocItemClick}> </TreeView>
        </div>
    {/if}
    <div bind:this={renderer}
         class = "content-renderer"
         on:keydown={handleKeyPress}
         on:click={handleLinkClick}>
    </div>
</div>
<svelte:window on:keyup={handleKeyPress} />

<style>
    .content-renderer {
        position: relative;
        column-fill: auto;
        table-layout: fixed;
        column-width: 500px;
        width: 100%;
        flex: 1;
        height: 95vh;
        padding: 40px;
        gap: 40px;
        overflow: hidden;
    }

    .book-reader {
        background: #fbf1c7;
        display: flex;
    }

    .toc {
        width: 20%;
        overflow-y:auto;
        overflow-x: clip;
        border-right-width: 10px;
        border-right-color: black;
    }
</style>