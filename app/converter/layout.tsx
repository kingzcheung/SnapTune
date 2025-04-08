export default function ConverterLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <section className="flex flex-col items-center justify-center w-full h-full">
      {children}
    </section>
  );
}
