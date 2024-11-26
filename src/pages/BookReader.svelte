<script lang="ts">
import {getContext, onMount} from "svelte";
    import {core} from "@tauri-apps/api";
    import TreeView from "../components/TreeView.svelte";
    import TocElem from "../components/TreeView.svelte"
    import BookParser from "../util/book_parser"
import {writeSetting} from "../util/styling"
import {invoke} from "@tauri-apps/api/core"
import {Mouse} from "lucide-svelte";
import { Window } from "@tauri-apps/api/window"
const { push ,pop} = getContext("stackview");

    export let bookPath     : string
    export let libraryPath  : string

    let renderer                = document.createElement("div");
	let bookReader              = document.createElement("div");
    let elems                   = document.createElement("div");
	let toc                     = document.createElement("div")

    let title                   = ""

    let contents: string[]      = []
    let tocItems: TocElem[]     = []
    let imgDict                 = {}
    let links                   = {}
    let section_hrefs           = {}

    let hash                    = 0
    let index                   = 0;
    let elemStartIndex          = 0
    let elemEndIndex            = 0;

    let showSearch              = false
    let showRightSideBar        = false
    let isResizing = false
let tocWidth: number = 250


    onMount(() => {
        let args = {bookPath: bookPath, libraryPath: libraryPath}
        invoke("get_book_content", args).then((cont) => {

		    title           = cont.title
            contents        = cont.contents
            index           = cont.section_index
            elemStartIndex  = cont.elem_index
            elemEndIndex    = cont.elem_index
            hash            = cont.hash
            tocItems        = cont.toc
            let hrefs       = cont.section_hrefs
            console.log(tocItems)
            console.log(hrefs)
            for (let style of cont.style_sheets) {apply_css(style, bookReader)}
			applyStyle(cont.user_style)
            for (let i = 0; i < hrefs.length; i++) {section_hrefs[hrefs[i]] = i}
            add_content(contents[index])
            renderForward()
            loadImages()
        }).catch(error => {console.error("Err: ", error)})
    });

	function apply_css(css: string, elem: HTMLElement) {
	    const style = document.createElement('style');
	    style.textContent = css;
	    elem.appendChild(style)
    }

	function applyStyle(style) {
        renderer.style.gap          = style["margins"];
		renderer.style.paddingLeft  = style["margins"];
		renderer.style.marginRight  = style["margins"];
		tocWidth                    = parseInt(style["toc-width"]);
		bookReader.style.fontSize   = style["font-size"];
		renderer.style.textAlign = "justify"
    }

    function loadImages() {
        invoke("get_images", {bookPath: bookPath}).then((imgs) => {
            for (let i = 0; i < imgs.length; i++) {imgDict[imgs[i][0]] = imgs[i][1]}
        })
    }

	function resetRenderer() {renderer.innerHTML = ""}

    function add_content(content: string) {
        links = {}
        let temp_div = document.createElement("div")
        temp_div.innerHTML = content;
        elems.innerHTML = ""
        let book_parser = new BookParser(links, imgDict, elems)
        for (let i = 0; i < temp_div.children.length; i++) {
            let child = (temp_div.children[i] as HTMLElement)
            book_parser.parse_elem(child)
        }
    }

    function writePosition() {
        const args = {libPath: libraryPath,hash: hash, sectionIndex: index, elemIndex: elemStartIndex}
        invoke("write_book_position", args).then(() => {})
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
	    resetRenderer()
        elemStartIndex = ++elemEndIndex;
        renderForward()
        writePosition()
    }

    function next_section() {
	    if (index == contents.length - 1) return
	    resetRenderer()
        index++;
        elemEndIndex = 0;
        elemStartIndex = elemEndIndex;
        add_content(contents[index])
        renderForward()
        tocItems.at(index).selected = true
        writePosition()
    }

    function prev() {
	    if (elemStartIndex <= 0) { return prev_chapter(); }
	    resetRenderer()
        elemEndIndex = --elemStartIndex;
        renderBackward()
        elemEndIndex++
        renderForward()
        writePosition()
    }

    function prev_chapter() {
	    if (index == 0) return;
	    resetRenderer()
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
                case "+"            : change_font_size(1); break
                case "-"            : change_font_size(-1); break
            }
        }
    }

    function change_font_size(delta: number) {
        let font_size = (+(bookReader.style.fontSize.replace("px", "")));
		font_size += delta
        bookReader.style.fontSize = font_size + 'px'
        writeSetting("epub-style", "font-size", font_size + "px")
    }

    function handleTocItemClick(value: string) {
        if (value.includes("www.")) return
	    resetRenderer()
        //let section_href    = value.split("#")[0]
        index               = section_hrefs[value.split("#")[0]]
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

	function handleMouseDown() {
	    isResizing = true
        document.body.style.webkitUserSelect = "none"
        document.body.style.cursor = "col-resize"
    }

	function handleMouseUp() {
        isResizing = false
        document.body.style.webkitUserSelect = ""
        document.body.style.cursor = "default"
    }

	function handleMouseMove(event: MouseEvent) {
	    if (isResizing) {
		    tocWidth += event.movementX
            writeSetting("epub-style","toc-width", tocWidth + "")
            event.preventDefault()
		}
    }

	window.addEventListener('mousemove', handleMouseMove);
	window.addEventListener('mouseup', handleMouseUp)
</script>


<div class="epub-root">
    <div class="epub-header">
        <button on:click={pop}> Back </button>
        <button on:click={() => showRightSideBar = !showRightSideBar}>Sidebar</button>
        <span>{title}</span>
    </div>

<div class = "book-reader" bind:this={bookReader}>
    {#if showRightSideBar}
        <div class="toc" bind:this={toc} style="width: {tocWidth}px">
            <TreeView items={tocItems} onItemClicked={handleTocItemClick} on:mousedown={handleMouseDown}> </TreeView>
        </div>
        <div class="resizer"
             on:mousedown={handleMouseDown}
            on:mouseenter= {() => {document.body.style.cursor = "col-resize"}}
            on:mouseleave= {() => {if(!isResizing) document.body.style.cursor = "default"}}>

        </div>
    {/if}
        <div bind:this={renderer}
             class = "content-renderer"
             on:keydown={handleKeyPress}
             on:click={handleLinkClick}>
        </div>
</div>
</div>
<svelte:window on:keyup={handleKeyPress} />

<style>
    body {
        height: 100%;
    }
    .content-renderer {
        column-fill: auto;
        table-layout: fixed;
        column-width: 30ch;
        text-justify: inter-word;
        overflow: hidden;
        line-height: 1.2em;
        flex-grow: 1;
        width: 1px;
    }

    .epub-root {margin: 0; padding: 0; height: 100vh}

    .book-reader {
        display: flex;
        font-family: Serif;
        overflow-y: hidden;
        height: calc(100vh - 32px)
    }

    .epub-header {
        height: 30px;
        border-bottom: 1px solid #aaa;
    }

    .resizer {
        width: 5px;
        border-right: 1px solid #aaa; /* Right border */
    }

    .toc {
        width: 1000px;
        font-size: 0.7em;
        padding-right: 10px;
        text-indent: 0px;

        display: flex;
        overflow-y: scroll;
        overflow-x: hidden;

        border-bottom-right-radius: 2px; /* Bottom right border radius */
        box-shadow: 0px 0px 1px rgba(0, 0, 0, 0.1);
    }
</style>