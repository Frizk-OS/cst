let completed = 0;
const benchmarks = BenchmarkSuite.CountBenchmarks();
let success = true;
function element(id) {
    const value = document.getElementById(id);
    if (!value) {
        throw new Error(`Missing element: ${id}`);
    }
    return value;
}
function ShowBox(name) {
    element(`Box-${name}`).style.visibility = 'visible';
    element('progress-bar').style.width = `${(++completed / benchmarks) * 100}%`;
}
function AddResult(name, result) {
    CtsReport(name, result, false);
    element(`Result-${name}`).innerHTML = String(result);
}
function AddError(name, error) {
    console.log(error);
    AddResult(name, error === 'TypedArrayUnsupported' ? '<b>Unsupported</b>' : '<b>Error</b>');
    success = false;
}
function AddScore(score) {
    element('main-banner').innerHTML = `Octane Score${success ? '' : ' (incomplete)'}: ${score}`;
    CtsReport(`Octane Score${success ? '' : ' (incomplete)'}`, score, true);
    element('progress-bar-container').style.visibility = 'hidden';
    element('bottom-text').style.visibility = 'visible';
    element('inside-anchor').removeChild(element('bar-appendix'));
}
function Run() {
    element('main-banner').innerHTML = 'Running Octane...';
    element('bar-appendix').innerHTML = '<br/><div class="progress progress-striped" id="progress-bar-container" style="visibility:hidden"><div class="bar" style="width: 0%;" id="progress-bar"></div></div>';
    const anchor = element('run-octane');
    const parent = element('main-container');
    parent.appendChild(element('inside-anchor'));
    parent.removeChild(anchor);
    element('startup-text').innerHTML = '';
    element('progress-bar-container').style.visibility = 'visible';
    BenchmarkSuite.RunSuites({
        NotifyStart: ShowBox,
        NotifyError: AddError,
        NotifyResult: AddResult,
        NotifyScore: AddScore,
    });
}
function CheckCompatibility() {
    const hasTypedArrays = typeof Uint8Array !== 'undefined'
        && typeof Float64Array !== 'undefined'
        && typeof new Uint8Array(0).subarray !== 'undefined';
    if (!hasTypedArrays) {
        console.log('Typed Arrays not supported');
        element('alertbox').style.display = 'block';
    }
    if (window.document.URL.indexOf('auto=1') >= 0) {
        Run();
    }
}
function Load() {
    setTimeout(CheckCompatibility, 200);
}
Object.assign(window, { AddError, AddResult, AddScore, CheckCompatibility, Load, Run, ShowBox });
