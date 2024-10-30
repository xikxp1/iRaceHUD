export function getBadgeColor(license: string) {
    switch (license.charAt(0)) {
        case "R":
            return "bg-error";
        case "D":
            return "bg-orange-600";
        case "C":
            return "bg-yellow-600";
        case "B":
            return "bg-green-700";
        case "A":
            return "bg-blue-800";
        case "P":
            return "bg-black";
        default:
            return "";
    }
}
