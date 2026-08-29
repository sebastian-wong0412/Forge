import { Link } from "react-router-dom";

export function OnboardingCard() {
  return (
    <section className="onboarding" aria-labelledby="onboarding-title">
      <h2 id="onboarding-title">开始使用 Forge</h2>
      <p>Forge 帮你把长期目标变成可以每天执行的任务。</p>
      <p>从一个周期开始，逐步建立你的工作系统。</p>
      <ol className="onboarding-steps">
        <li>
          <strong>创建一个周期</strong>
          <span>例如：Q3 2026 / 秋季计划 / 产品冲刺</span>
        </li>
        <li>
          <strong>建立你的目标</strong>
          <span>明确这个周期最重要的成果</span>
        </li>
        <li>
          <strong>创建项目和任务</strong>
          <span>把目标拆成真正可以执行的工作</span>
        </li>
        <li>
          <strong>在「今日」页面执行</strong>
          <span>安排任务、开始工作、完成任务</span>
        </li>
      </ol>
      <Link to="/cycles" className="btn btn-primary">
        创建我的第一个周期
      </Link>
    </section>
  );
}
