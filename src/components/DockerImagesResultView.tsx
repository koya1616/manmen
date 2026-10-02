import { useTranslation } from "react-i18next";
import type { ImageSnapshot } from "../types";

export function DockerImagesResultView({ result }: { result: ImageSnapshot }) {
  const { t } = useTranslation();

  if (result.images.length === 0) {
    return <p className="muted">{t("dockerImages.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("dockerImages.repo")}</th>
            <th>{t("dockerImages.tag")}</th>
            <th>{t("dockerImages.size")}</th>
            <th>{t("dockerImages.created")}</th>
          </tr>
        </thead>
        <tbody>
          {result.images.map((img) => (
            <tr key={img.id}>
              <td className="mono">{img.repository}</td>
              <td className="mono">{img.tag}</td>
              <td className="num mono">{img.size}</td>
              <td className="mono">{img.created_since}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
