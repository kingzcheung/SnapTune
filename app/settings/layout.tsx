export default function SettingsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <section className="flex flex-col items-center justify-center w-full h-full py-2 px-4">
      {children}
    </section>
  );
}
