/* Copyright (C) 2026 The AOSP and FrizkOS */

package android.test;

import android.app.KeyguardManager;
import android.content.Context;
import android.content.pm.PackageManager;
import android.os.Bundle;
import android.util.Log;

import java.io.File;
import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.net.Authenticator;
import java.net.CookieHandler;
import java.net.ResponseCache;
import java.util.Locale;

import javax.net.ssl.HostnameVerifier;
import javax.net.ssl.HttpsURLConnection;
import javax.net.ssl.SSLSocketFactory;

import junit.framework.AssertionFailedError;
import junit.framework.Test;
import junit.framework.TestCase;
import junit.framework.TestListener;

/**
 * Modernized, SDK-compliant InstrumentationCtsTestRunner for Android 4.4.4 KitKat CTS.
 * Extends standard Android InstrumentationTestRunner and sets up environment properties,
 * disables the keyguard, and configures context classloaders.
 */
public class InstrumentationCtsTestRunner extends InstrumentationTestRunner {

    private static final String TAG = "InstrumentationCtsTestRunner";

    private TestEnvironment mEnvironment;

    @Override
    public void onCreate(Bundle arguments) {
        Context targetContext = getTargetContext();
        if (targetContext != null) {
            File cacheDir = targetContext.getCacheDir();
            if (cacheDir != null) {
                String cachePath = cacheDir.getAbsolutePath();
                System.setProperty("user.language", "en");
                System.setProperty("user.region", "US");
                System.setProperty("java.home", cachePath);
                System.setProperty("user.home", cachePath);
                System.setProperty("java.io.tmpdir", cachePath);
                System.setProperty("user.dir", cachePath);
            }
        }

        mEnvironment = new TestEnvironment();

        try {
            if (getContext().checkCallingOrSelfPermission(android.Manifest.permission.DISABLE_KEYGUARD)
                    == PackageManager.PERMISSION_GRANTED) {
                Log.i(TAG, "Disabling keyguard for CTS test run");
                KeyguardManager keyguardManager =
                    (KeyguardManager) getContext().getSystemService(Context.KEYGUARD_SERVICE);
                if (keyguardManager != null) {
                    keyguardManager.newKeyguardLock("cts").disableKeyguard();
                }
            } else {
                Log.i(TAG, "Test lacks permission to disable keyguard");
            }
        } catch (Throwable t) {
            Log.w(TAG, "Failed to disable keyguard: " + t.getMessage());
        }

        super.onCreate(arguments);
    }

    @Override
    protected AndroidTestRunner getAndroidTestRunner() {
        AndroidTestRunner runner = super.getAndroidTestRunner();
        if (runner != null) {
            runner.addTestListener(new TestListener() {
                private Class<?> lastClass;

                @Override
                public void startTest(Test test) {
                    if (test != null) {
                        Class<?> clazz = test.getClass();
                        if (clazz != lastClass) {
                            lastClass = clazz;
                        }
                        Thread.currentThread().setContextClassLoader(clazz.getClassLoader());
                    }
                    if (mEnvironment != null) {
                        mEnvironment.reset();
                    }
                }

                @Override
                public void endTest(Test test) {
                    if (test instanceof TestCase) {
                        cleanup((TestCase) test);
                    }
                }

                @Override
                public void addError(Test test, Throwable t) {}

                @Override
                public void addFailure(Test test, AssertionFailedError t) {}

                private void cleanup(TestCase test) {
                    Class<?> clazz = test.getClass();
                    while (clazz != null && clazz != TestCase.class) {
                        Field[] fields = clazz.getDeclaredFields();
                        for (Field f : fields) {
                            if (!f.getType().isPrimitive() && !Modifier.isStatic(f.getModifiers())) {
                                try {
                                    f.setAccessible(true);
                                    f.set(test, null);
                                } catch (Throwable ignored) {}
                            }
                        }
                        clazz = clazz.getSuperclass();
                    }
                }
            });
        }
        return runner;
    }

    static class TestEnvironment {
        private final Locale mDefaultLocale;
        private final String mUserHome;
        private final String mJavaIoTmpDir;
        private final HostnameVerifier mHostnameVerifier;
        private final SSLSocketFactory mSslSocketFactory;

        TestEnvironment() {
            mDefaultLocale = Locale.getDefault();
            mUserHome = System.getProperty("user.home", "");
            mJavaIoTmpDir = System.getProperty("java.io.tmpdir", "");
            mHostnameVerifier = HttpsURLConnection.getDefaultHostnameVerifier();
            mSslSocketFactory = HttpsURLConnection.getDefaultSSLSocketFactory();
        }

        void reset() {
            Locale.setDefault(mDefaultLocale);
            System.setProperty("user.home", mUserHome);
            System.setProperty("java.io.tmpdir", mJavaIoTmpDir);
            try {
                Authenticator.setDefault(null);
                CookieHandler.setDefault(null);
                ResponseCache.setDefault(null);
                HttpsURLConnection.setDefaultHostnameVerifier(mHostnameVerifier);
                HttpsURLConnection.setDefaultSSLSocketFactory(mSslSocketFactory);
            } catch (Throwable ignored) {}
        }
    }
}
