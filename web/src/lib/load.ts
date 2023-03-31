import { element } from "svelte/internal";
import { Facebook, Github, Contact } from "./data";
let url: string = "http://localhost:8080/api/userinfo/Akhil"
export async function links_loader(): Promise<Array<(Facebook | Github | Contact)>> {
    let response = await fetch(url);
    let data: Array<any> = await response.json();
    console.log(data)
    let data_obj = data.map((element: any, index: number, array) => {
        if ("Contact" in element) {
            return new Contact(element.Contact.mobile_number, element.Contact.address)
        } else if ("Instagram" in element) {
            return new Facebook(element.Instagram.link)
        } else if ("Github" in element) {
            return new Github(element.Github.link)
        }
    })
    console.log(data_obj)
    return data_obj
}