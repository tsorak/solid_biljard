export default async function getMotd() {
  const res = await fetch("/api/motd");

  await new Promise((res, _) => {
    setTimeout(() => res(), 1000);
  });

  if (res.ok === false) {
    return {};
  }

  const data = await res.json();

  return data;
}
