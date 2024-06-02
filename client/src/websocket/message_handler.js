/**
 * @param {MessageEvent<any>} ev
 */
export default function handle_message(ev) {
  let msg = "" + ev.data;
  msg = msg.split(" ", 2);

  const [type, _data] = (msg.length > 1) ? msg : [msg[0], ""];

  switch (type) {
    case "refresh":
      window.location.reload();
      break;

    default:
      console.log("Received '" + type + "' message");
      break;
  }
}
