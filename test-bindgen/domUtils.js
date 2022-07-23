export const appendStringToBody = (value) => {
    const text = document.createTextNode(value);
    document.body.appendChild(text);
};

export const addTwoNumbers = (v1, v2) => {
    return v1+v2 ;
}