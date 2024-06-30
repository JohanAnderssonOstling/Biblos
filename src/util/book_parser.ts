interface parseState {links: {}, imgDict: {}, elems: HTMLElement}
export default
class BookParser {
    count                       = 0
    private readonly links      : {}
    private readonly imgDict    : {}
    private elems               : HTMLElement
    constructor(links: {}, imgDict: {}, elems: HTMLElement) {
        this.links = links;     this.imgDict = imgDict;     this.elems = elems
    }
    parse_elem(elem: HTMLElement) {
        let tagName = elem.tagName.toLowerCase();
        if (elem.getAttribute("id")) this.links[elem.getAttribute("id")] = this.count
        switch (tagName) {
            case "body"     :   this.parse_children(elem);  break
            case "section"  :   this.parse_children(elem);  break
            case "dl"       :   this.parse_children(elem);  break
            case "div"      :   this.parse_div(elem);       break
            case "a"        :   this.links[elem.nodeValue]; this.parse_children(elem);  break
            default         :   this.append_elem(elem);     break
        }
    }

    parse_div(elem: HTMLElement) {
        if (!this.containsDiv(elem)){this.append_elem(elem)}
        else                        {this.parse_children(elem)}
    }

    parse_children(elem : HTMLElement) {
        for (let i = 0; i < elem.children.length; i++) {
            this.parse_elem(elem.children[i] as HTMLElement)
        }
    }

    append_elem(elem: HTMLElement) {
        this.modify_elem(elem)
        this.count++
        this.elems.append(elem.cloneNode(true))
    }

    containsDiv(elem: HTMLElement) {
        for (let i = 0; i < elem.children.length; i++) {
            let child = elem.children[i] as HTMLElement
            let tagName = child.tagName.toLowerCase()
            if (" div p dl section ".includes(" " + tagName + " ") || this.containsDiv(child)) return true;
        }
        return false;
    }

    modify_elem(elem: HTMLElement) {
        if (elem.getAttribute("id"))    this.links[elem.getAttribute("id")] = this.count
        if (elem.nodeName == "A")       {return;}

        let tagName = elem.tagName.toLowerCase()
        if (tagName == "img") {
            let imgPath     = elem.src.split("/").slice(3).toString().replace(",", "/")
            elem.src        = this.imgDict[imgPath]
        }
        if (tagName == "svg" || tagName == "image") {
            let xlink       = elem.getAttributeNS("http://www.w3.org/1999/xlink", "xlink:href");
            if (xlink != null) {
                let imgPath = xlink.split("/").slice(3).toString().replace(",", "/")
                elem.setAttributeNS("http://www.w3.org/1999/xlink", "xlink:href", imgPath);
            }
        }
        for (let i = 0; i < elem.children.length; i++) {
            this.modify_elem(elem.children[i] as HTMLElement)
        }
    }
}
